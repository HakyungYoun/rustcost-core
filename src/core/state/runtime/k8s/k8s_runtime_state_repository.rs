use std::sync::Arc;
use tokio::sync::RwLock;

use crate::core::state::runtime::k8s::k8s_runtime_state::K8sRuntimeState;
use crate::core::state::runtime::k8s::k8s_runtime_state_repository_trait::K8sRuntimeStateRepositoryTrait;

pub struct K8sRuntimeStateRepository {
    state: Arc<RwLock<K8sRuntimeState>>,
}

impl K8sRuntimeStateRepository {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(K8sRuntimeState::default())),
        }
    }

    pub fn shared(self) -> Arc<Self> {
        Arc::new(self)
    }
}

#[async_trait::async_trait]
impl K8sRuntimeStateRepositoryTrait for K8sRuntimeStateRepository {
    async fn get(&self) -> K8sRuntimeState {
        self.state.read().await.clone()
    }

    async fn set(&self, new_state: K8sRuntimeState) {
        let mut state = self.state.write().await;
        *state = new_state;
    }

    async fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut K8sRuntimeState) + Send + Sync,
    {
        let mut state = self.state.write().await;
        f(&mut state);
    }
}
