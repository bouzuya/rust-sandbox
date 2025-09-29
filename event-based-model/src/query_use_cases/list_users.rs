pub struct ListUsersDeps {
    pub user_reader: std::sync::Arc<dyn crate::readers::UserReader + Send + Sync>,
}

#[derive(Debug)]
pub struct ListUsersInput;

#[derive(Debug)]
pub struct ListUsersOutput {
    pub items: Vec<ListUsersOutputItem>,
}

#[derive(Debug)]
pub struct ListUsersOutputItem {
    pub id: String,
    pub name: String,
    pub version: u32,
}

impl std::convert::From<crate::query_models::QueryUser> for ListUsersOutputItem {
    fn from(
        crate::query_models::QueryUser { id, name, version }: crate::query_models::QueryUser,
    ) -> Self {
        Self { id, name, version }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ListUsersError {
    #[error("list users")]
    ListUsers(#[source] crate::readers::UserReaderError),
}

pub async fn list_users(
    ListUsersDeps { user_reader }: ListUsersDeps,
    ListUsersInput: ListUsersInput,
) -> Result<ListUsersOutput, ListUsersError> {
    let users = user_reader
        .list()
        .await
        .map_err(ListUsersError::ListUsers)?;
    let items = users
        .into_iter()
        .map(ListUsersOutputItem::from)
        .collect::<Vec<ListUsersOutputItem>>();
    Ok(ListUsersOutput { items })
}
