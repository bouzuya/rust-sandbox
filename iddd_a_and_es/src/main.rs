use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use ulid::Ulid;

fn main() {
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
    );

    appliation_service_method(&mut event_store, aggregate_id);

    println!("{:?}", event_store);
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
    aggregate_id: MyAggregateId, // ?
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
    fn from_events(events: &[MyEvent]) -> Self {
        events
            .iter()
            .fold(None, |acc, event| {
                let e: MyAggregateEvent = serde_json::from_str(event.data.as_str()).unwrap();
                match e {
                    MyAggregateEvent::Created => match acc {
                        Some(_) => panic!(),
                        None => Some(Self {
                            id: event.aggregate_id,
                            version: MyVersion(1),
                            value: false,
                        }),
                    },
                    MyAggregateEvent::ValueTrue => match acc {
                        None => panic!(),
                        Some(mut x) => {
                            if x.id != event.aggregate_id {
                                panic!();
                            }
                            x.version = x.version.next();
                            x.value = true;
                            Some(x)
                        }
                    },
                    MyAggregateEvent::ValueFalse => match acc {
                        None => panic!(),
                        Some(mut x) => {
                            if x.id != event.aggregate_id {
                                panic!();
                            }
                            x.version = x.version().next();
                            x.value = false;
                            Some(x)
                        }
                    },
                    _ => unreachable!(),
                }
            })
            .unwrap()
    }

    fn id(&self) -> MyAggregateId {
        self.id
    }

    fn update_a(&self) -> Vec<MyEvent> {
        vec![MyEvent {
            aggregate_id: self.id(),
            version: self.version().next(),
            data: serde_json::to_string(&MyAggregateEvent::ValueTrue).unwrap(),
        }]
    }

    fn update_b(&self) -> Vec<MyEvent> {
        vec![MyEvent {
            aggregate_id: self.id(),
            version: self.version().next(),
            data: serde_json::to_string(&MyAggregateEvent::ValueFalse).unwrap(),
        }]
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
    fn find_by_id(&self, aggregate_id: MyAggregateId) -> Vec<MyEvent> {
        // SELECT * FROM events WHERE aggregate_id = ? ORDER BY version;
        self.events
            .iter()
            .filter(|e| e.aggregate_id == aggregate_id)
            .cloned()
            .collect()
    }

    fn save(&mut self, aggregate_id: MyAggregateId, version: MyVersion, events: &[MyEvent]) {
        // UPDATE aggregates SET version = ? WHERE id = ? AND version = ?
        let uncommitted_version = events.last().unwrap().version;
        let committed_version = self
            .aggregates
            .entry(aggregate_id)
            .or_insert(uncommitted_version);
        if *committed_version != version {
            panic!();
        }
        *committed_version = uncommitted_version;
        // INSERT INTO events (aggregate_id, version, data) VALUES (?, ?, ?);
        self.events.extend(events.iter().cloned());
    }
}

fn appliation_service_method(event_store: &mut MyEventStore, aggregate_id: MyAggregateId) {
    // Repository::find_by_id(&self, aggregate_id: AggregateId) -> Aggregate
    let aggregate = {
        let committed_events = event_store.find_by_id(aggregate_id);
        MyAggregate::from_events(&committed_events)
    };

    let uncommitted_events = aggregate.update_a();

    // Repository::save(&self, aggregate: Aggregate)
    event_store.save(aggregate.id(), aggregate.version(), &uncommitted_events);
}
