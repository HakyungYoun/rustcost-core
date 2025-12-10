use axum::extract::{Path, Query, State};
use axum::Json;
use k8s_openapi::api::apps::v1::Deployment;
use crate::api::util::json::to_json;
use crate::api::dto::ApiResponse;
use crate::api::dto::info_dto::PaginationQuery;
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::api::dto::paginated_response::PaginatedResponse;

pub struct InfoK8sDeploymentController;

impl InfoK8sDeploymentController {
    pub async fn get_k8s_deployments(
        State(state): State<AppState>,
        Query(pagination): Query<PaginationQuery>,
    ) -> Result<Json<ApiResponse<PaginatedResponse<Deployment>>>, AppError> {
        to_json(
            state
                .info_k8s_service
                .get_k8s_deployments_paginated(pagination.limit, pagination.offset)
                .await,
        )
    }

    pub async fn get_k8s_deployment(
        Path((namespace, name)): Path<(String, String)>,
        State(state): State<AppState>,
    ) -> Result<Json<ApiResponse<Deployment>>, AppError> {
        to_json(
            state
                .info_k8s_service
                .get_k8s_deployment(namespace, name)
                .await,
        )
    }
}
