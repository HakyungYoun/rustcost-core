use axum::{routing::{get, post}, Router};
use crate::api::controller::state::alert::alert_state_controller::AlertStateController;
use crate::api::controller::state::k8s::k8s_state_controller::K8sStateController;
use crate::app_state::AppState;


pub fn state_routes() -> Router<AppState> {
    Router::new()
        // --- K8s Runtime State ---
        .route("/k8s", get(K8sStateController::get_full))
        .route("/k8s/summary", get(K8sStateController::get_summary))

        // --- Alerts Runtime State ---
        .route("/alerts", get(AlertStateController::get_active))
        .route("/alerts/all", get(AlertStateController::get_all))
        .route("/alerts/fire", post(AlertStateController::fire))
        .route("/alerts/resolve/{id}", post(AlertStateController::resolve))
}
