
use crate::core::persistence::metrics::k8s::container::day::metric_container_day_repository::MetricContainerDayRepository;
use crate::core::persistence::metrics::k8s::container::hour::metric_container_hour_repository::MetricContainerHourRepository;
use crate::core::persistence::metrics::k8s::container::minute::metric_container_minute_repository::MetricContainerMinuteRepository;
use crate::core::persistence::metrics::k8s::node::day::metric_node_day_repository::MetricNodeDayRepository;
use crate::core::persistence::metrics::k8s::node::hour::metric_node_hour_repository::MetricNodeHourRepository;
use crate::core::persistence::metrics::k8s::node::minute::metric_node_minute_repository::MetricNodeMinuteRepository;
use crate::core::persistence::metrics::k8s::pod::day::metric_pod_day_repository::MetricPodDayRepository;
use crate::core::persistence::metrics::k8s::pod::hour::metric_pod_hour_repository::MetricPodHourRepository;
use crate::core::persistence::metrics::k8s::pod::minute::metric_pod_minute_repository::MetricPodMinuteRepository;

pub enum K8sMetricRepositoryVariant {
    // Node
    NodeMinute(MetricNodeMinuteRepository),
    NodeHour(MetricNodeHourRepository),
    NodeDay(MetricNodeDayRepository),

    // Pod
    PodMinute(MetricPodMinuteRepository),
    PodHour(MetricPodHourRepository),
    PodDay(MetricPodDayRepository),

    // Container
    ContainerMinute(MetricContainerMinuteRepository),
    ContainerHour(MetricContainerHourRepository),
    ContainerDay(MetricContainerDayRepository),
}
