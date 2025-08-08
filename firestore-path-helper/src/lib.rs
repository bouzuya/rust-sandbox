mod repo_id;
mod user_id;

mod repo {
    // firestore_path_helper::firestore_path_helper!("users/{user_id}/repos/{repo_id}", user_id = UserId, repo_id = RepoId);
    // assert_eq!(collection(CollectionParams { user_id }), CollectionPath::from_str("users/user123/repos")?);
    // assert_eq!(document(DocumentParams { user_id, repo_id }), DocumentPath::from_str("users/user123/repos/repo456")?);
    // assert_eq!(document_id(DocumentPath::from_str("users/user123/repos/repo456")?)?, RepoId::from_str("repo456")?);

    use firestore_path::{CollectionPath, DocumentPath};

    use std::str::FromStr as _;

    use crate::{repo_id::RepoId, user_id::UserId};

    #[derive(Debug, thiserror::Error)]
    #[error("")]
    pub struct Error;

    const DOCUMENT_PATH_TEMPLATE: &str = "users/{user_id}/repos/{repo_id}";
    const COLLECTION_ID: &str = "repos";

    pub struct CollectionParams {
        pub user_id: UserId,
    }

    pub fn collection(CollectionParams { user_id }: CollectionParams) -> CollectionPath {
        CollectionPath::from_str(&format!("users/{user_id}/repos"))
            .expect("Failed to create collection path")
    }

    pub struct DocumentParams {
        pub user_id: UserId,
        pub repo_id: RepoId,
    }

    pub fn document(DocumentParams { user_id, repo_id }: DocumentParams) -> DocumentPath {
        DocumentPath::from_str(&format!("users/{user_id}/repos/{repo_id}"))
            .expect("Failed to create document path")
    }

    pub fn document_id(document_path: DocumentPath) -> Result<RepoId, Error> {
        let document_id = document_path.document_id().to_string();
        let user_id = RepoId::from_str(&document_id).map_err(|_| Error)?;
        if document_path.parent().collection_id().to_string() != COLLECTION_ID {
            return Err(Error);
        }
        // TODO:
        Ok(user_id)
    }
}

mod repo2 {
    // firestore_path_helper::firestore_path_helper!("users/{user_id}/repos/{repo_id}", user_id = UserId, repo_id = RepoId);
    // assert_eq!(collection(CollectionParams { user_id }), CollectionPath::from_str("users/user123/repos")?);
    // assert_eq!(document(DocumentParams { user_id, repo_id }), DocumentPath::from_str("users/user123/repos/repo456")?);
    // assert_eq!(document_id(DocumentPath::from_str("users/user123/repos/repo456")?)?, RepoId::from_str("repo456")?);

    use firestore_path::{CollectionPath, DocumentPath};

    use std::str::FromStr as _;

    use crate::{repo_id::RepoId, user_id::UserId};

    #[derive(Debug, thiserror::Error)]
    #[error("")]
    pub struct Error;

    const DOCUMENT_PATH_TEMPLATE: &str = "users/{user_id}/repos/{repo_id}";
    const COLLECTION_ID: &str = "repos";

    pub struct Collection {
        pub user_id: UserId,
    }

    impl Collection {
        pub fn path(&self) -> CollectionPath {
            CollectionPath::from_str(&format!("users/{}/repos", self.user_id))
                .expect("Failed to create collection path")
        }
    }

    pub struct Document {
        pub user_id: UserId,
        pub repo_id: RepoId,
    }

    impl Document {
        pub fn path(&self) -> DocumentPath {
            DocumentPath::from_str(&format!("users/{}/repos/{}", self.user_id, self.repo_id))
                .expect("Failed to create document path")
        }
    }

    pub fn document_id(document_path: DocumentPath) -> Result<RepoId, Error> {
        let document_id = document_path.document_id().to_string();
        let user_id = RepoId::from_str(&document_id).map_err(|_| Error)?;
        if document_path.parent().collection_id().to_string() != COLLECTION_ID {
            return Err(Error);
        }
        // TODO:
        Ok(user_id)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use crate::{repo_id::RepoId, user_id::UserId};

    use super::*;

    #[test]
    fn test_repo() -> anyhow::Result<()> {
        let user_id = UserId::from_str("user123")?;
        let collection_path = repo::collection(repo::CollectionParams { user_id });
        assert_eq!(collection_path.to_string(), "users");

        let user_id = UserId::from_str("user123")?;
        let repo_id = RepoId::from_str("repo456")?;
        let document_path = repo::document(repo::DocumentParams { user_id, repo_id });
        assert_eq!(document_path.to_string(), "users/user123/repos/repo456");

        let repo_id = repo2::document_id(document_path)?;
        assert_eq!(repo_id, RepoId::from_str("repo456")?);
        Ok(())
    }

    #[test]
    fn test_repo2() -> anyhow::Result<()> {
        let user_id = UserId::from_str("user123")?;
        let collection_path = repo2::Collection { user_id }.path();
        assert_eq!(collection_path.to_string(), "users");

        let user_id = UserId::from_str("user123")?;
        let repo_id = RepoId::from_str("repo456")?;
        let document_path = repo2::Document { user_id, repo_id }.path();
        assert_eq!(document_path.to_string(), "users/user123/repos/repo456");

        let repo_id = repo2::document_id(document_path)?;
        assert_eq!(repo_id, RepoId::from_str("repo456")?);
        Ok(())
    }
}
