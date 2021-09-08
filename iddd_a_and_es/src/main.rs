use std::collections::BTreeMap;

use ulid::Ulid;

fn main() {
    let mut event_store = MyEventStore::default();
    let aggregate_id = MyAggregateId(Ulid::new());
    event_store.save(
        aggregate_id,
        MyVersion(1),
        &[MyEvent {
            aggregate_id,
            version: MyVersion(1),
            data: "created".to_string(),
        }],
    );

    appliation_service_method(&mut event_store, aggregate_id);

    println!("{:?}", event_store);
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct MyVersion(usize);

#[derive(Clone, Debug)]
struct MyEvent {
    aggregate_id: MyAggregateId, // ?
    version: MyVersion,
    data: String,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct MyAggregateId(Ulid);

struct MyAggregate {
    id: MyAggregateId,
    version: MyVersion,
}

impl MyAggregate {
    fn from_events(events: &[MyEvent]) -> Self {
        // events.is_empty() ?
        Self {
            id: events.first().unwrap().aggregate_id,
            version: events.last().unwrap().version,
        }
    }

    fn id(&self) -> MyAggregateId {
        self.id
    }

    fn update(&self) -> Vec<MyEvent> {
        vec![MyEvent {
            aggregate_id: self.id(),
            version: MyVersion(self.version.0 + 1),
            data: "updated".to_string(),
        }]
    }

    fn version(&self) -> MyVersion {
        self.version
    }
}

#[derive(Clone, Debug, Default)]
struct MyEventStore {
    aggregates: BTreeMap<MyAggregateId, MyVersion>,
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
        // UPDATE aggregates SET version = ? WHERE aggregate_id = ? AND version = ?
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

    let uncommitted_events = aggregate.update();

    // Repository::save(&self, aggregate: Aggregate)
    event_store.save(aggregate.id(), aggregate.version(), &uncommitted_events);
}
