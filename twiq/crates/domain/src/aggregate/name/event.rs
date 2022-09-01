pub struct FetchRequested;
pub struct FetchResultReceived;

pub enum Event {
    FetchRequested(FetchRequested),
    FetchResultReceived(FetchResultReceived),
}
