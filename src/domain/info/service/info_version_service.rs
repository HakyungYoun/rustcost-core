use anyhow::Result;
use serde_json::{json, Value};
use crate::core::persistence::info::fixed::version::info_version_api_repository_trait::InfoVersionApiRepository;
use crate::core::persistence::info::fixed::version::info_version_entity::InfoVersionEntity;
use crate::core::persistence::info::fixed::version::info_version_repository::InfoVersionRepositoryImpl;

pub async fn get_info_versions() -> Result<InfoVersionEntity> {
    let repo = InfoVersionRepositoryImpl::new();
    get_info_versions_with_repo(&repo).await
}

pub async fn upsert_info_version() -> Result<Value> {
    let repo = InfoVersionRepositoryImpl::new();
    upsert_info_version_with_repo(&repo).await
}

async fn get_info_versions_with_repo<R: InfoVersionApiRepository>(
    repo: &R,
) -> Result<InfoVersionEntity> {
    let entity = repo.read()?;
    Ok(entity)
}

async fn upsert_info_version_with_repo<R: InfoVersionApiRepository>(
    repo: &R,
) -> Result<Value> {
    // Until we introduce a DTO for updating, ensure the file exists
    // by reading current value and rewriting it.
    let current = repo.read().unwrap_or_default();
    repo.update(&current)?;

    Ok(json!({
        "message": "Version updated successfully",
    }))
}
