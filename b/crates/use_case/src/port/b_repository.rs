use std::path::{Path, PathBuf};

use entity::{BId, BMeta};

pub trait BRepository {
    // TODO: hide path ?
    fn find_by_content_path(&self, path: &Path) -> anyhow::Result<BId>;

    // TODO: hide path ?
    fn find_by_meta_path(&self, path: &Path) -> anyhow::Result<BId>;

    fn find_content(&self, id: BId) -> anyhow::Result<Option<String>>;

    fn find_ids(&self, date: &str) -> anyhow::Result<Vec<BId>>;

    fn find_meta(&self, id: BId) -> anyhow::Result<Option<BMeta>>;

    fn to_content_path_buf(&self, id: &BId) -> PathBuf;

    fn to_meta_path_buf(&self, id: &BId) -> PathBuf;
}

pub trait HasBRepository {
    type BRepository: BRepository;

    fn b_repository(&self) -> &Self::BRepository;
}
