// #[derive(Clone, Debug, Eq, PartialEq)]
// enum Event {
//     CustomerLocked,
//     CustomerUnlocked,
// }

// trait Identity {}

// trait IEventStore {
//     fn load_event_stream<I: Identity>(&self, id: &I) -> EventStream;
//     fn load_event_stream_with_limit<I: Identity>(
//         &self,
//         id: &I,
//         skip_events: usize,
//         max_count: usize,
//     ) -> EventStream;
//     fn append_to_stream<I: Identity>(&self, id: I, expected_version: usize, events: &[Event]);
// }

// struct EventStore {
//     store: Vec<Event>,
// }

// impl IEventStore for EventStore {
//     fn load_event_stream<I: Identity>(&self, id: &I) -> EventStream {
//         todo!()
//     }

//     fn load_event_stream_with_limit<I: Identity>(
//         &self,
//         id: &I,
//         skip_events: usize,
//         max_count: usize,
//     ) -> EventStream {
//         todo!()
//     }

//     fn append_to_stream<I: Identity>(&self, id: I, expected_version: usize, events: &[Event]) {
//         todo!()
//     }
// }

// struct EventStream {
//     events: Vec<Event>,
//     version: usize,
// }

// struct IPricingService;

// struct Customer {
//     changes: Vec<Event>,
//     consumption_locked: bool,
// }

// impl Customer {
//     pub fn new(events: Vec<Event>) -> Self {
//         events.into_iter().fold(
//             Self {
//                 changes: vec![],
//                 consumption_locked: false,
//             },
//             |mut customer, event| {
//                 customer.mutate(&event);
//                 customer
//             },
//         )
//     }

//     pub fn lock_for_account_overdraft(&self, comment: String, pricing_service: &IPricingService) {
//         todo!()
//     }

//     // pub fn lock_customer(&self, reason: String) {
//     //     if !self.consumption_locked {
//     //         self.apply(Event::CustomerLocked(state.id, reason));
//     //     }
//     // }

//     pub fn changes(&self) -> &Vec<Event> {
//         &self.changes
//     }

//     fn apply(&mut self, event: Event) {
//         self.mutate(&event);
//         self.changes.push(event);
//     }

//     fn mutate(&mut self, event: &Event) {
//         match event {
//             Event::CustomerLocked => self.consumption_locked = true,
//             Event::CustomerUnlocked => self.consumption_locked = false,
//         }
//     }
// }

// struct CustomerId;

// impl Identity for CustomerId {}

// struct CustomerApplicationService {
//     event_store: EventStore,
//     pricing_service: IPricingService,
// }

// impl CustomerApplicationService {
//     fn new(event_store: EventStore, pricing_service: IPricingService) -> Self {
//         Self {
//             event_store,
//             pricing_service,
//         }
//     }

//     fn lock_for_account_overdraft(&self, customer_id: CustomerId, comment: String) {
//         let stream = self.event_store.load_event_stream(&customer_id);
//         let customer = Customer::new(stream.events);
//         customer.lock_for_account_overdraft(comment, &self.pricing_service);
//         self.event_store
//             .append_to_stream(customer_id, stream.version, customer.changes());
//     }
// }
