#[cfg(test)]
mod mock_list_presenter;
#[cfg(test)]
mod mock_task_repository;

#[cfg(test)]
pub use self::mock_list_presenter::MockListPresenter;
#[cfg(test)]
pub use self::mock_task_repository::MockTaskRepository;
