use async_trait::async_trait;
use crate::core::state::runtime::k8s::k8s_runtime_state::K8sRuntimeState;

#[async_trait]
pub trait K8sRuntimeStateRepositoryTrait: Send + Sync {
    async fn get(&self) -> K8sRuntimeState;

    async fn set(&self, state: K8sRuntimeState);

    async fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut K8sRuntimeState) + Send + Sync;
}
