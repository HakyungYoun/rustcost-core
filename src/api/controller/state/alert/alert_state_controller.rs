use axum::{
    extract::{State, Path, Json},
};
use serde_json::{json, Value};
use crate::api::dto::ApiResponse;
use crate::api::util::json::to_json;
use crate::app_state::AppState;
use crate::core::state::runtime::alerts::alert_runtime_state_repository_trait::AlertRuntimeStateRepositoryTrait;
use crate::errors::AppError;

pub struct AlertStateController;

impl AlertStateController {

    pub async fn get_active(
        State(state): State<AppState>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let alerts = state.alerts.active_alerts().await;
        to_json(Ok(json!({ "active_alerts": alerts })))
    }

    pub async fn get_all(
        State(state): State<AppState>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let all = state.alerts.repo.get().await.alerts;
        to_json(Ok(json!({ "alerts": all })))
    }

    pub async fn fire(
        State(state): State<AppState>,
        Json(payload): Json<Value>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let id = payload["id"].as_str().unwrap_or("").to_string();
        let message = payload["message"].as_str().unwrap_or("").to_string();
        let severity = payload["severity"].as_str().unwrap_or("info").to_string();

        state.alerts.fire_alert(id, message, severity).await;

        to_json(Ok(json!({ "status": "ok" })))
    }

    pub async fn resolve(
        Path(id): Path<String>,
        State(state): State<AppState>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        state.alerts.resolve_alert(&id).await;

        to_json(Ok(json!({ "resolved": id })))
    }
}
