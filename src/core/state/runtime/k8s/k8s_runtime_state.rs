use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Runtime-only Kubernetes state for RustCost.
///
/// This state is NOT persisted to disk.
/// It is kept purely in memory and rebuilt on process start.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct K8sRuntimeState {
    // ===== Timestamps =====
    pub last_discovered_at: Option<DateTime<Utc>>,
    pub last_error_at: Option<DateTime<Utc>>,

    // ===== Kubernetes Objects =====
    pub nodes: Vec<String>,
    pub namespaces: Vec<String>,
    pub deployments: Vec<String>,
    pub pods: Vec<String>,
    pub containers: Vec<String>,

    // ===== Counts =====
    pub node_count: u32,
    pub namespace_count: u32,
    pub deployment_count: u32,
    pub pod_count: u32,
    pub container_count: u32,

    // ===== Optional error =====
    pub last_error_message: Option<String>,
}

impl Default for K8sRuntimeState {
    fn default() -> Self {
        Self {
            last_discovered_at: None,
            last_error_at: None,

            nodes: Vec::new(),
            namespaces: Vec::new(),
            deployments: Vec::new(),
            pods: Vec::new(),
            containers: Vec::new(),

            node_count: 0,
            namespace_count: 0,
            deployment_count: 0,
            pod_count: 0,
            container_count: 0,

            last_error_message: None,
        }
    }
}

impl K8sRuntimeState {
    pub fn update(
        &mut self,
        nodes: Vec<String>,
        namespaces: Vec<String>,
        deployments: Vec<String>,
        pods: Vec<String>,
        containers: Vec<String>,
    ) {
        self.node_count = nodes.len() as u32;
        self.namespace_count = namespaces.len() as u32;
        self.deployment_count = deployments.len() as u32;
        self.pod_count = pods.len() as u32;
        self.container_count = containers.len() as u32;

        self.nodes = nodes;
        self.namespaces = namespaces;
        self.deployments = deployments;
        self.pods = pods;
        self.containers = containers;

        self.last_discovered_at = Some(Utc::now());
        self.last_error_message = None;
        self.last_error_at = None;
    }

    pub fn mark_error(&mut self, msg: String) {
        self.last_error_message = Some(msg);
        self.last_error_at = Some(Utc::now());
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
