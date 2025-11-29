use std::sync::Arc;

use crate::core::state::runtime::k8s::k8s_runtime_state::K8sRuntimeState;
use crate::core::state::runtime::k8s::k8s_runtime_state_repository_trait::K8sRuntimeStateRepositoryTrait;

pub struct K8sRuntimeStateManager<R: K8sRuntimeStateRepositoryTrait> {
    pub(crate) repo: Arc<R>,
}

impl<R: K8sRuntimeStateRepositoryTrait> K8sRuntimeStateManager<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    pub async fn set_state(&self, state: K8sRuntimeState) {
        self.repo.set(state).await;
    }

    pub async fn update_discovery(
        &self,
        nodes: Vec<String>,
        namespaces: Vec<String>,
        deployments: Vec<String>,
        pods: Vec<String>,
        containers: Vec<String>,
    ) {
        self.repo
            .update(|s| {
                s.update(
                    nodes.clone(),
                    namespaces.clone(),
                    deployments.clone(),
                    pods.clone(),
                    containers.clone(),
                );
            })
            .await;
    }

    pub async fn mark_error(&self, err: String) {
        self.repo.update(|s| s.mark_error(err)).await;
    }
}
