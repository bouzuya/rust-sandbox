use anyhow::Context;
use entity::{BId, BMeta};

use crate::{BRepository, HasBRepository};

pub trait ViewUseCase: HasBRepository {
    fn handle(&self, id: BId) -> anyhow::Result<(BMeta, String)> {
        let repository = self.b_repository();
        let meta = repository.find_meta(id)?;
        let meta = meta.with_context(|| "b meta not found")?;
        let content = repository.find_content(id)?;
        let content = content.with_context(|| "b content not found")?;
        Ok((meta, content))
    }
}

impl<T: HasBRepository> ViewUseCase for T {}

pub trait HasViewUseCase {
    type ViewUseCase: ViewUseCase;

    fn view_use_case(&self) -> &Self::ViewUseCase;
}
