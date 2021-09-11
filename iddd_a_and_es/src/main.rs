// TODO: ID に対応した Event がほしい。 AggregateId -> Vec<AggregateEvent>。 別の集約の型を追加すると良さそう。二種類の ID を取らないといけなくなるのでそこが検討される
// TODO: ID 以外の条件で対象を取得する必要が出たらどうするのだろうか
// TODO: 集約以外の単位でのイベント

use std::collections::BTreeMap;

use anyhow::Context;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

fn main() -> anyhow::Result<()> {
    let mut event_store = MyEventStore::default();
    let aggregate_id = MyAggregateId(Ulid::new());
    let version = MyVersion(1);
    event_store.save(
        aggregate_id,
        version,
        &[MyEvent {
            aggregate_id,
            version,
            data: serde_json::to_string(&MyAggregateEvent::Created).unwrap(),
        }],
    )?;

    appliation_service_method(&mut event_store, aggregate_id)?;

    println!("{:?}", event_store);

    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct MyVersion(usize);

impl MyVersion {
    fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

#[derive(Clone, Debug)]
struct MyEvent {
    aggregate_id: MyAggregateId,
    version: MyVersion,
    data: String,
}

#[derive(Deserialize, Serialize)]
enum MyAggregateEvent {
    Created,
    ValueTrue,
    ValueFalse,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct MyAggregateId(Ulid);

struct MyAggregate {
    id: MyAggregateId,
    version: MyVersion,
    value: bool,
}

impl MyAggregate {
    fn from_events(events: &[MyEvent]) -> anyhow::Result<Self> {
        events
            .iter()
            .try_fold(None, |acc: Option<MyAggregate>, event| {
                let e: MyAggregateEvent = serde_json::from_str(event.data.as_str())?;
                match e {
                    MyAggregateEvent::Created => match acc {
                        None => Ok(Some(Self {
                            id: event.aggregate_id,
                            version: MyVersion(1),
                            value: false,
                        })),
                        Some(_) => anyhow::bail!("created"),
                    },
                    MyAggregateEvent::ValueTrue => match acc {
                        None => anyhow::bail!("no created"),
                        Some(mut x) => {
                            if x.id != event.aggregate_id {
                                anyhow::bail!("other aggregate event");
                            }
                            x.version = x.version.next();
                            x.value = true;
                            Ok(Some(x))
                        }
                    },
                    MyAggregateEvent::ValueFalse => match acc {
                        None => anyhow::bail!("no created"),
                        Some(mut x) => {
                            if x.id != event.aggregate_id {
                                anyhow::bail!("other aggregate event");
                            }
                            x.version = x.version().next();
                            x.value = false;
                            Ok(Some(x))
                        }
                    },
                }
            })
            .and_then(|op| op.context("empty"))
    }

    fn id(&self) -> MyAggregateId {
        self.id
    }

    fn update_a(&self) -> anyhow::Result<Vec<MyEvent>> {
        Ok(vec![MyEvent {
            aggregate_id: self.id(),
            version: self.version().next(),
            data: serde_json::to_string(&MyAggregateEvent::ValueTrue).unwrap(),
        }])
    }

    fn update_b(&self) -> anyhow::Result<Vec<MyEvent>> {
        Ok(vec![MyEvent {
            aggregate_id: self.id(),
            version: self.version().next(),
            data: serde_json::to_string(&MyAggregateEvent::ValueFalse).unwrap(),
        }])
    }

    fn version(&self) -> MyVersion {
        self.version
    }
}

#[derive(Clone, Debug, Default)]
struct MyEventStore {
    // 集約別の最新のバージョンを保持する
    // `SELECT MAX(version) FROM events WHERE aggregate_id = ?` で取得できるが排他制御の観点で使用する
    //
    // ```sql
    // CREATE TABLE aggregates (
    //     id      BLOB    NOT NULL, -- ULID
    //     version INTEGER NOT NULL,
    //     PRIMARY KEY (id)
    // )
    // ```
    aggregates: BTreeMap<MyAggregateId, MyVersion>,
    // 集約別のイベントを保持する
    //
    // ```sql
    // CREATE TABLE events (
    //     aggregate_id BLOB    NOT NULL, -- ULID
    //     version      INTEGER NOT NULL,
    //     data         TEXT    NOT NULL, -- or BLOB
    //     PRIMARY KEY (aggregate_id, version),
    //     FOREIGN KEY (aggregate_id) REFERENCES aggregates (id)
    // )
    // ```
    events: Vec<MyEvent>,
}

impl MyEventStore {
    fn find_by_id(&self, aggregate_id: MyAggregateId) -> anyhow::Result<Vec<MyEvent>> {
        // SELECT * FROM events WHERE aggregate_id = ? ORDER BY version;
        Ok(self
            .events
            .iter()
            .filter(|e| e.aggregate_id == aggregate_id)
            .cloned()
            .collect())
    }

    fn save(
        &mut self,
        aggregate_id: MyAggregateId,
        version: MyVersion,
        events: &[MyEvent],
    ) -> anyhow::Result<()> {
        // UPDATE aggregates SET version = ? WHERE id = ? AND version = ?
        let uncommitted_version = events.last().unwrap().version;
        let committed_version = self
            .aggregates
            .entry(aggregate_id)
            .or_insert(uncommitted_version);
        if *committed_version != version {
            anyhow::bail!("concurrent exception");
        }
        *committed_version = uncommitted_version;
        // INSERT INTO events (aggregate_id, version, data) VALUES (?, ?, ?);
        self.events.extend(events.iter().cloned());
        Ok(())
    }
}

fn appliation_service_method(
    event_store: &mut MyEventStore,
    aggregate_id: MyAggregateId,
) -> anyhow::Result<()> {
    // Repository::find_by_id(&self, aggregate_id: AggregateId) -> Aggregate
    let aggregate = {
        let committed_events = event_store.find_by_id(aggregate_id)?;
        MyAggregate::from_events(&committed_events)?
    };

    let uncommitted_events = aggregate.update_a()?;

    // Repository::save(&self, aggregate: Aggregate) -> Result<()>
    event_store.save(aggregate.id(), aggregate.version(), &uncommitted_events)?;

    Ok(())
}
