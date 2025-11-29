use std::sync::Arc;
use chrono::Utc;

use crate::core::state::runtime::alerts::alert_runtime_state::{AlertEvent, AlertRuntimeState};
use crate::core::state::runtime::alerts::alert_runtime_state_repository_trait::AlertRuntimeStateRepositoryTrait;

pub struct AlertRuntimeStateManager<R: AlertRuntimeStateRepositoryTrait> {
    pub(crate) repo: Arc<R>,
}

impl<R: AlertRuntimeStateRepositoryTrait> AlertRuntimeStateManager<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    pub async fn fire_alert(&self, id: String, message: String, severity: String) {
        let alert = AlertEvent {
            id,
            message,
            severity,
            created_at: Utc::now(),
            last_updated_at: Utc::now(),
            active: true,
        };

        self.repo.update(|s| s.add_or_update_alert(alert)).await;
    }

    pub async fn resolve_alert(&self, id: &str) {
        self.repo.update(|s| s.resolve_alert(id)).await;
    }

    pub async fn reset(&self) {
        self.repo.set(AlertRuntimeState::default()).await;
    }

    pub async fn active_alerts(&self) -> Vec<AlertEvent> {
        let s = self.repo.get().await;
        s.active_alerts()
    }
}
