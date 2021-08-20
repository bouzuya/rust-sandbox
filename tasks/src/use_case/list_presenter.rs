use crate::entity::Task;

pub trait ListPresenter {
    fn complete(&self, tasks: &[Task]);
}
