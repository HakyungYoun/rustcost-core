use axum::extract::{Query, State};
use axum::Json;
use serde_json::Value;
use crate::api::dto::{metrics_dto::RangeQuery, ApiResponse};
use crate::api::util::json::to_json;
use crate::app_state::AppState;
use crate::errors::AppError;

pub struct K8sClusterMetricsController;

impl K8sClusterMetricsController {
    pub async fn get_metric_k8s_cluster_raw(
        State(state): State<AppState>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {

        state.k8s_state.ensure_resynced().await?;
        let node_names = state.k8s_state.get_nodes().await;

        to_json(
            state
                .metric_service
                .get_metric_k8s_cluster_raw(q, node_names)
                .await,
        )
    }

    pub async fn get_metric_k8s_cluster_raw_summary(
        State(state): State<AppState>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {

        state.k8s_state.ensure_resynced().await?;
        let node_names = state.k8s_state.get_nodes().await;

        to_json(
            state
                .metric_service
                .get_metric_k8s_cluster_raw_summary(q, node_names)
                .await,
        )
    }

    pub async fn get_metric_k8s_cluster_cost(
        State(state): State<AppState>,
        Query(q): Query<RangeQuery>
    ) -> Result<Json<ApiResponse<Value>>, AppError> {

        state.k8s_state.ensure_resynced().await?;
        let node_names = state.k8s_state.get_nodes().await;

        to_json(state.metric_service.get_metric_k8s_cluster_cost(q, node_names).await)
    }

    pub async fn get_metric_k8s_cluster_cost_summary(
        State(state): State<AppState>,
        Query(q): Query<RangeQuery>
    ) -> Result<Json<ApiResponse<Value>>, AppError> {

        state.k8s_state.ensure_resynced().await?;
        let node_names = state.k8s_state.get_nodes().await;

        to_json(
            state
                .metric_service
                .get_metric_k8s_cluster_cost_summary(q, node_names)
                .await,
        )
    }

    pub async fn get_metric_k8s_cluster_cost_trend(
        State(state): State<AppState>,
        Query(q): Query<RangeQuery>
    ) -> Result<Json<ApiResponse<Value>>, AppError> {

        state.k8s_state.ensure_resynced().await?;
        let node_names = state.k8s_state.get_nodes().await;

        to_json(
            state
                .metric_service
                .get_metric_k8s_cluster_cost_trend(q, node_names)
                .await,
        )
    }

    pub async fn get_metric_k8s_cluster_raw_efficiency(
        State(state): State<AppState>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {

        state.k8s_state.ensure_resynced().await?;
        let node_names = state.k8s_state.get_nodes().await;

        to_json(
            state
                .metric_service
                .get_metric_k8s_cluster_raw_efficiency(q, node_names)
                .await,
        )
    }
}
