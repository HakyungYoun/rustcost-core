use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertEvent {
    pub id: String,
    pub message: String,
    pub severity: String,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub active: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AlertRuntimeState {
    pub alerts: Vec<AlertEvent>,
}

impl AlertRuntimeState {
    pub fn add_or_update_alert(&mut self, new_alert: AlertEvent) {
        if let Some(existing) = self.alerts.iter_mut().find(|a| a.id == new_alert.id) {

            // Keep original created_at — this is important!
            existing.message = new_alert.message;
            existing.severity = new_alert.severity;
            existing.active = new_alert.active;
            existing.last_updated_at = Utc::now();

        } else {
            self.alerts.push(new_alert);
        }
    }

    pub fn resolve_alert(&mut self, id: &str) {
        if let Some(a) = self.alerts.iter_mut().find(|a| a.id == id) {
            a.active = false;
            a.last_updated_at = Utc::now();
        }
    }

    pub fn active_alerts(&self) -> Vec<AlertEvent> {
        self.alerts.iter().filter(|a| a.active).cloned().collect()
    }

    /// Delete resolved alerts older than `max_age`.
    pub fn prune_resolved_older_than(&mut self, max_age: Duration) {
        let cutoff = Utc::now() - max_age;
        self.alerts.retain(|a| a.active || a.last_updated_at > cutoff);
    }

    /// Limit number of alerts; remove oldest (by last_updated_at)
    pub fn prune_by_max_len(&mut self, max_len: usize) {
        if self.alerts.len() <= max_len {
            return;
        }

        self.alerts.sort_by_key(|a| a.last_updated_at);
        let excess = self.alerts.len() - max_len;
        self.alerts.drain(0..excess);
    }
}
