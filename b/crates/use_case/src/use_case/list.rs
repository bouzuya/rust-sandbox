use anyhow::Context;
use entity::BMeta;

use crate::{BRepository, HasBRepository, Query};

pub trait ListUseCase: HasBRepository {
    fn handle(&self, query: &Query) -> anyhow::Result<Vec<BMeta>> {
        let repository = self.b_repository();
        let mut bmetas = vec![];
        let bids = repository.find_ids(query.date.as_str())?;
        for bid in bids {
            let bmeta = repository.find_meta(bid)?.context("no meta error")?;
            match &query.tags {
                Some(ref tags) => {
                    if tags
                        .iter()
                        .all(|tag| bmeta.tags.iter().any(|s| s.as_str() == tag))
                    {
                        bmetas.push(bmeta);
                    }
                }
                None => bmetas.push(bmeta),
            }
        }
        Ok(bmetas)
    }
}

impl<T: HasBRepository> ListUseCase for T {}

pub trait HasListUseCase {
    type ListUseCase: ListUseCase;

    fn list_use_case(&self) -> &Self::ListUseCase;
}
