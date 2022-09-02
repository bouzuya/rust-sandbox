pub struct Created;
pub struct FetchRequested;
pub struct FetchResultReceived;

pub enum Event {
    Created(Created),
    FetchRequested(FetchRequested),
    FetchResultReceived(FetchResultReceived),
}
