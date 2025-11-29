use axum::{extract::{State}, Json};
use serde_json::{json, Value};
use crate::api::dto::ApiResponse;
use crate::api::util::json::to_json;
use crate::app_state::AppState;
use crate::core::state::runtime::k8s::k8s_runtime_state_repository_trait::K8sRuntimeStateRepositoryTrait;
use crate::errors::AppError;

pub struct K8sStateController;

impl K8sStateController {
    pub async fn get_full(
        State(state): State<AppState>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let s = state.k8s_state.repo.get().await;
        to_json(Ok(json!(s)))
    }

    pub async fn get_summary(
        State(state): State<AppState>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let s = state.k8s_state.repo.get().await;

        to_json(Ok(json!({
            "nodes": s.node_count,
            "namespaces": s.namespace_count,
            "deployments": s.deployment_count,
            "pods": s.pod_count,
            "containers": s.container_count,
            "last_discovered_at": s.last_discovered_at,
            "last_error_at": s.last_error_at,
            "last_error_message": s.last_error_message,
        })))
    }
}
