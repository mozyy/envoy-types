/// Proto representation of certificate details. Admin endpoint uses this wrapper for `/certs` to
/// display certificate information. See :ref:`/certs <operations_admin_interface_certs>` for more
/// information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Certificates {
    /// List of certificates known to an Envoy.
    #[prost(message, repeated, tag = "1")]
    pub certificates: ::prost::alloc::vec::Vec<Certificate>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Certificate {
    /// Details of CA certificate.
    #[prost(message, repeated, tag = "1")]
    pub ca_cert: ::prost::alloc::vec::Vec<CertificateDetails>,
    /// Details of Certificate Chain
    #[prost(message, repeated, tag = "2")]
    pub cert_chain: ::prost::alloc::vec::Vec<CertificateDetails>,
}
/// \[\#next-free-field: 8\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateDetails {
    /// Path of the certificate.
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// Certificate Serial Number.
    #[prost(string, tag = "2")]
    pub serial_number: ::prost::alloc::string::String,
    /// List of Subject Alternate names.
    #[prost(message, repeated, tag = "3")]
    pub subject_alt_names: ::prost::alloc::vec::Vec<SubjectAlternateName>,
    /// Minimum of days until expiration of certificate and it's chain.
    #[prost(uint64, tag = "4")]
    pub days_until_expiration: u64,
    /// Indicates the time from which the certificate is valid.
    #[prost(message, optional, tag = "5")]
    pub valid_from: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
    /// Indicates the time at which the certificate expires.
    #[prost(message, optional, tag = "6")]
    pub expiration_time: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
    /// Details related to the OCSP response associated with this certificate, if any.
    #[prost(message, optional, tag = "7")]
    pub ocsp_details: ::core::option::Option<certificate_details::OcspDetails>,
}
/// Nested message and enum types in `CertificateDetails`.
pub mod certificate_details {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OcspDetails {
        /// Indicates the time from which the OCSP response is valid.
        #[prost(message, optional, tag = "1")]
        pub valid_from: ::core::option::Option<
            super::super::super::super::google::protobuf::Timestamp,
        >,
        /// Indicates the time at which the OCSP response expires.
        #[prost(message, optional, tag = "2")]
        pub expiration: ::core::option::Option<
            super::super::super::super::google::protobuf::Timestamp,
        >,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubjectAlternateName {
    /// Subject Alternate Name.
    #[prost(oneof = "subject_alternate_name::Name", tags = "1, 2, 3")]
    pub name: ::core::option::Option<subject_alternate_name::Name>,
}
/// Nested message and enum types in `SubjectAlternateName`.
pub mod subject_alternate_name {
    /// Subject Alternate Name.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Name {
        #[prost(string, tag = "1")]
        Dns(::prost::alloc::string::String),
        #[prost(string, tag = "2")]
        Uri(::prost::alloc::string::String),
        #[prost(string, tag = "3")]
        IpAddress(::prost::alloc::string::String),
    }
}
/// Proto representation of an Envoy Counter or Gauge value.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimpleMetric {
    /// Type of the metric represented.
    #[prost(enumeration = "simple_metric::Type", tag = "1")]
    pub r#type: i32,
    /// Current metric value.
    #[prost(uint64, tag = "2")]
    pub value: u64,
    /// Name of the metric.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `SimpleMetric`.
pub mod simple_metric {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        Counter = 0,
        Gauge = 1,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Counter => "COUNTER",
                Type::Gauge => "GAUGE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "COUNTER" => Some(Self::Counter),
                "GAUGE" => Some(Self::Gauge),
                _ => None,
            }
        }
    }
}
/// Admin endpoint uses this wrapper for `/clusters` to display cluster status information.
/// See :ref:`/clusters <operations_admin_interface_clusters>` for more information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Clusters {
    /// Mapping from cluster name to each cluster's status.
    #[prost(message, repeated, tag = "1")]
    pub cluster_statuses: ::prost::alloc::vec::Vec<ClusterStatus>,
}
/// Details an individual cluster's current status.
/// \[\#next-free-field: 9\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterStatus {
    /// Name of the cluster.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Denotes whether this cluster was added via API or configured statically.
    #[prost(bool, tag = "2")]
    pub added_via_api: bool,
    /// The success rate threshold used in the last interval.
    /// If
    /// :ref:`outlier_detection.split_external_local_origin_errors<envoy_v3_api_field_config.cluster.v3.OutlierDetection.split_external_local_origin_errors>`
    /// is `false`, all errors: externally and locally generated were used to calculate the threshold.
    /// If
    /// :ref:`outlier_detection.split_external_local_origin_errors<envoy_v3_api_field_config.cluster.v3.OutlierDetection.split_external_local_origin_errors>`
    /// is `true`, only externally generated errors were used to calculate the threshold.
    /// The threshold is used to eject hosts based on their success rate. See
    /// :ref:`Cluster outlier detection <arch_overview_outlier_detection>` documentation for details.
    ///
    /// Note: this field may be omitted in any of the three following cases:
    ///
    /// 1. There were not enough hosts with enough request volume to proceed with success rate based
    ///    outlier ejection.
    /// 1. The threshold is computed to be \< 0 because a negative value implies that there was no
    ///    threshold for that interval.
    /// 1. Outlier detection is not enabled for this cluster.
    #[prost(message, optional, tag = "3")]
    pub success_rate_ejection_threshold: ::core::option::Option<
        super::super::r#type::v3::Percent,
    >,
    /// Mapping from host address to the host's current status.
    #[prost(message, repeated, tag = "4")]
    pub host_statuses: ::prost::alloc::vec::Vec<HostStatus>,
    /// The success rate threshold used in the last interval when only locally originated failures were
    /// taken into account and externally originated errors were treated as success.
    /// This field should be interpreted only when
    /// :ref:`outlier_detection.split_external_local_origin_errors<envoy_v3_api_field_config.cluster.v3.OutlierDetection.split_external_local_origin_errors>`
    /// is `true`. The threshold is used to eject hosts based on their success rate.
    /// See :ref:`Cluster outlier detection <arch_overview_outlier_detection>` documentation for
    /// details.
    ///
    /// Note: this field may be omitted in any of the three following cases:
    ///
    /// 1. There were not enough hosts with enough request volume to proceed with success rate based
    ///    outlier ejection.
    /// 1. The threshold is computed to be \< 0 because a negative value implies that there was no
    ///    threshold for that interval.
    /// 1. Outlier detection is not enabled for this cluster.
    #[prost(message, optional, tag = "5")]
    pub local_origin_success_rate_ejection_threshold: ::core::option::Option<
        super::super::r#type::v3::Percent,
    >,
    /// :ref:`Circuit breaking <arch_overview_circuit_break>` settings of the cluster.
    #[prost(message, optional, tag = "6")]
    pub circuit_breakers: ::core::option::Option<
        super::super::config::cluster::v3::CircuitBreakers,
    >,
    /// Observability name of the cluster.
    #[prost(string, tag = "7")]
    pub observability_name: ::prost::alloc::string::String,
    /// The :ref:`EDS service name <envoy_v3_api_field_config.cluster.v3.Cluster.EdsClusterConfig.service_name>` if the cluster is an EDS cluster.
    #[prost(string, tag = "8")]
    pub eds_service_name: ::prost::alloc::string::String,
}
/// Current state of a particular host.
/// \[\#next-free-field: 10\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HostStatus {
    /// Address of this host.
    #[prost(message, optional, tag = "1")]
    pub address: ::core::option::Option<super::super::config::core::v3::Address>,
    /// List of stats specific to this host.
    #[prost(message, repeated, tag = "2")]
    pub stats: ::prost::alloc::vec::Vec<SimpleMetric>,
    /// The host's current health status.
    #[prost(message, optional, tag = "3")]
    pub health_status: ::core::option::Option<HostHealthStatus>,
    /// Request success rate for this host over the last calculated interval.
    /// If
    /// :ref:`outlier_detection.split_external_local_origin_errors<envoy_v3_api_field_config.cluster.v3.OutlierDetection.split_external_local_origin_errors>`
    /// is `false`, all errors: externally and locally generated were used in success rate
    /// calculation. If
    /// :ref:`outlier_detection.split_external_local_origin_errors<envoy_v3_api_field_config.cluster.v3.OutlierDetection.split_external_local_origin_errors>`
    /// is `true`, only externally generated errors were used in success rate calculation.
    /// See :ref:`Cluster outlier detection <arch_overview_outlier_detection>` documentation for
    /// details.
    ///
    /// Note: the message will not be present if host did not have enough request volume to calculate
    /// success rate or the cluster did not have enough hosts to run through success rate outlier
    /// ejection.
    #[prost(message, optional, tag = "4")]
    pub success_rate: ::core::option::Option<super::super::r#type::v3::Percent>,
    /// The host's weight. If not configured, the value defaults to 1.
    #[prost(uint32, tag = "5")]
    pub weight: u32,
    /// The hostname of the host, if applicable.
    #[prost(string, tag = "6")]
    pub hostname: ::prost::alloc::string::String,
    /// The host's priority. If not configured, the value defaults to 0 (highest priority).
    #[prost(uint32, tag = "7")]
    pub priority: u32,
    /// Request success rate for this host over the last calculated
    /// interval when only locally originated errors are taken into account and externally originated
    /// errors were treated as success.
    /// This field should be interpreted only when
    /// :ref:`outlier_detection.split_external_local_origin_errors<envoy_v3_api_field_config.cluster.v3.OutlierDetection.split_external_local_origin_errors>`
    /// is `true`.
    /// See :ref:`Cluster outlier detection <arch_overview_outlier_detection>` documentation for
    /// details.
    ///
    /// Note: the message will not be present if host did not have enough request volume to calculate
    /// success rate or the cluster did not have enough hosts to run through success rate outlier
    /// ejection.
    #[prost(message, optional, tag = "8")]
    pub local_origin_success_rate: ::core::option::Option<
        super::super::r#type::v3::Percent,
    >,
    /// locality of the host.
    #[prost(message, optional, tag = "9")]
    pub locality: ::core::option::Option<super::super::config::core::v3::Locality>,
}
/// Health status for a host.
/// \[\#next-free-field: 9\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HostHealthStatus {
    /// The host is currently failing active health checks.
    #[prost(bool, tag = "1")]
    pub failed_active_health_check: bool,
    /// The host is currently considered an outlier and has been ejected.
    #[prost(bool, tag = "2")]
    pub failed_outlier_check: bool,
    /// The host is currently being marked as degraded through active health checking.
    #[prost(bool, tag = "4")]
    pub failed_active_degraded_check: bool,
    /// The host has been removed from service discovery, but is being stabilized due to active
    /// health checking.
    #[prost(bool, tag = "5")]
    pub pending_dynamic_removal: bool,
    /// The host has not yet been health checked.
    #[prost(bool, tag = "6")]
    pub pending_active_hc: bool,
    /// The host should be excluded from panic, spillover, etc. calculations because it was explicitly
    /// taken out of rotation via protocol signal and is not meant to be routed to.
    #[prost(bool, tag = "7")]
    pub excluded_via_immediate_hc_fail: bool,
    /// The host failed active HC due to timeout.
    #[prost(bool, tag = "8")]
    pub active_hc_timeout: bool,
    /// Health status as reported by EDS. Note: only HEALTHY and UNHEALTHY are currently supported
    /// here.
    /// \[\#comment:TODO(mrice32): pipe through remaining EDS health status possibilities.\]
    #[prost(enumeration = "super::super::config::core::v3::HealthStatus", tag = "3")]
    pub eds_health_status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFailureState {
    /// What the component configuration would have been if the update had succeeded.
    /// This field may not be populated by xDS clients due to storage overhead.
    #[prost(message, optional, tag = "1")]
    pub failed_configuration: ::core::option::Option<
        super::super::super::google::protobuf::Any,
    >,
    /// Time of the latest failed update attempt.
    #[prost(message, optional, tag = "2")]
    pub last_update_attempt: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
    /// Details about the last failed update attempt.
    #[prost(string, tag = "3")]
    pub details: ::prost::alloc::string::String,
    /// This is the version of the rejected resource.
    /// \[\#not-implemented-hide:\]
    #[prost(string, tag = "4")]
    pub version_info: ::prost::alloc::string::String,
}
/// Envoy's listener manager fills this message with all currently known listeners. Listener
/// configuration information can be used to recreate an Envoy configuration by populating all
/// listeners as static listeners or by returning them in a LDS response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenersConfigDump {
    /// This is the :ref:`version_info <envoy_v3_api_field_service.discovery.v3.DiscoveryResponse.version_info>` in the
    /// last processed LDS discovery response. If there are only static bootstrap listeners, this field
    /// will be "".
    #[prost(string, tag = "1")]
    pub version_info: ::prost::alloc::string::String,
    /// The statically loaded listener configs.
    #[prost(message, repeated, tag = "2")]
    pub static_listeners: ::prost::alloc::vec::Vec<
        listeners_config_dump::StaticListener,
    >,
    /// State for any warming, active, or draining listeners.
    #[prost(message, repeated, tag = "3")]
    pub dynamic_listeners: ::prost::alloc::vec::Vec<
        listeners_config_dump::DynamicListener,
    >,
}
/// Nested message and enum types in `ListenersConfigDump`.
pub mod listeners_config_dump {
    /// Describes a statically loaded listener.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StaticListener {
        /// The listener config.
        #[prost(message, optional, tag = "1")]
        pub listener: ::core::option::Option<
            super::super::super::super::google::protobuf::Any,
        >,
        /// The timestamp when the Listener was last successfully updated.
        #[prost(message, optional, tag = "2")]
        pub last_updated: ::core::option::Option<
            super::super::super::super::google::protobuf::Timestamp,
        >,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DynamicListenerState {
        /// This is the per-resource version information. This version is currently taken from the
        /// :ref:`version_info <envoy_v3_api_field_service.discovery.v3.DiscoveryResponse.version_info>` field at the time
        /// that the listener was loaded. In the future, discrete per-listener versions may be supported
        /// by the API.
        #[prost(string, tag = "1")]
        pub version_info: ::prost::alloc::string::String,
        /// The listener config.
        #[prost(message, optional, tag = "2")]
        pub listener: ::core::option::Option<
            super::super::super::super::google::protobuf::Any,
        >,
        /// The timestamp when the Listener was last successfully updated.
        #[prost(message, optional, tag = "3")]
        pub last_updated: ::core::option::Option<
            super::super::super::super::google::protobuf::Timestamp,
        >,
    }
    /// Describes a dynamically loaded listener via the LDS API.
    /// \[\#next-free-field: 7\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DynamicListener {
        /// The name or unique id of this listener, pulled from the DynamicListenerState config.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// The listener state for any active listener by this name.
        /// These are listeners that are available to service data plane traffic.
        #[prost(message, optional, tag = "2")]
        pub active_state: ::core::option::Option<DynamicListenerState>,
        /// The listener state for any warming listener by this name.
        /// These are listeners that are currently undergoing warming in preparation to service data
        /// plane traffic. Note that if attempting to recreate an Envoy configuration from a
        /// configuration dump, the warming listeners should generally be discarded.
        #[prost(message, optional, tag = "3")]
        pub warming_state: ::core::option::Option<DynamicListenerState>,
        /// The listener state for any draining listener by this name.
        /// These are listeners that are currently undergoing draining in preparation to stop servicing
        /// data plane traffic. Note that if attempting to recreate an Envoy configuration from a
        /// configuration dump, the draining listeners should generally be discarded.
        #[prost(message, optional, tag = "4")]
        pub draining_state: ::core::option::Option<DynamicListenerState>,
        /// Set if the last update failed, cleared after the next successful update.
        /// The `error_state` field contains the rejected version of this particular
        /// resource along with the reason and timestamp. For successfully updated or
        /// acknowledged resource, this field should be empty.
        #[prost(message, optional, tag = "5")]
        pub error_state: ::core::option::Option<super::UpdateFailureState>,
        /// The client status of this resource.
        /// \[\#not-implemented-hide:\]
        #[prost(enumeration = "super::ClientResourceStatus", tag = "6")]
        pub client_status: i32,
    }
}
/// Envoy's cluster manager fills this message with all currently known clusters. Cluster
/// configuration information can be used to recreate an Envoy configuration by populating all
/// clusters as static clusters or by returning them in a CDS response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClustersConfigDump {
    /// This is the :ref:`version_info <envoy_v3_api_field_service.discovery.v3.DiscoveryResponse.version_info>` in the
    /// last processed CDS discovery response. If there are only static bootstrap clusters, this field
    /// will be "".
    #[prost(string, tag = "1")]
    pub version_info: ::prost::alloc::string::String,
    /// The statically loaded cluster configs.
    #[prost(message, repeated, tag = "2")]
    pub static_clusters: ::prost::alloc::vec::Vec<clusters_config_dump::StaticCluster>,
    /// The dynamically loaded active clusters. These are clusters that are available to service
    /// data plane traffic.
    #[prost(message, repeated, tag = "3")]
    pub dynamic_active_clusters: ::prost::alloc::vec::Vec<
        clusters_config_dump::DynamicCluster,
    >,
    /// The dynamically loaded warming clusters. These are clusters that are currently undergoing
    /// warming in preparation to service data plane traffic. Note that if attempting to recreate an
    /// Envoy configuration from a configuration dump, the warming clusters should generally be
    /// discarded.
    #[prost(message, repeated, tag = "4")]
    pub dynamic_warming_clusters: ::prost::alloc::vec::Vec<
        clusters_config_dump::DynamicCluster,
    >,
}
/// Nested message and enum types in `ClustersConfigDump`.
pub mod clusters_config_dump {
    /// Describes a statically loaded cluster.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StaticCluster {
        /// The cluster config.
        #[prost(message, optional, tag = "1")]
        pub cluster: ::core::option::Option<
            super::super::super::super::google::protobuf::Any,
        >,
        /// The timestamp when the Cluster was last updated.
        #[prost(message, optional, tag = "2")]
        pub last_updated: ::core::option::Option<
            super::super::super::super::google::protobuf::Timestamp,
        >,
    }
    /// Describes a dynamically loaded cluster via the CDS API.
    /// \[\#next-free-field: 6\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DynamicCluster {
        /// This is the per-resource version information. This version is currently taken from the
        /// :ref:`version_info <envoy_v3_api_field_service.discovery.v3.DiscoveryResponse.version_info>` field at the time
        /// that the cluster was loaded. In the future, discrete per-cluster versions may be supported by
        /// the API.
        #[prost(string, tag = "1")]
        pub version_info: ::prost::alloc::string::String,
        /// The cluster config.
        #[prost(message, optional, tag = "2")]
        pub cluster: ::core::option::Option<
            super::super::super::super::google::protobuf::Any,
        >,
        /// The timestamp when the Cluster was last updated.
        #[prost(message, optional, tag = "3")]
        pub last_updated: ::core::option::Option<
            super::super::super::super::google::protobuf::Timestamp,
        >,
        /// Set if the last update failed, cleared after the next successful update.
        /// The `error_state` field contains the rejected version of this particular
        /// resource along with the reason and timestamp. For successfully updated or
        /// acknowledged resource, this field should be empty.
        /// \[\#not-implemented-hide:\]
        #[prost(message, optional, tag = "4")]
        pub error_state: ::core::option::Option<super::UpdateFailureState>,
        /// The client status of this resource.
        /// \[\#not-implemented-hide:\]
        #[prost(enumeration = "super::ClientResourceStatus", tag = "5")]
        pub client_status: i32,
    }
}
/// Envoy's RDS implementation fills this message with all currently loaded routes, as described by
/// their RouteConfiguration objects. Static routes that are either defined in the bootstrap configuration
/// or defined inline while configuring listeners are separated from those configured dynamically via RDS.
/// Route configuration information can be used to recreate an Envoy configuration by populating all routes
/// as static routes or by returning them in RDS responses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutesConfigDump {
    /// The statically loaded route configs.
    #[prost(message, repeated, tag = "2")]
    pub static_route_configs: ::prost::alloc::vec::Vec<
        routes_config_dump::StaticRouteConfig,
    >,
    /// The dynamically loaded route configs.
    #[prost(message, repeated, tag = "3")]
    pub dynamic_route_configs: ::prost::alloc::vec::Vec<
        routes_config_dump::DynamicRouteConfig,
    >,
}
/// Nested message and enum types in `RoutesConfigDump`.
pub mod routes_config_dump {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StaticRouteConfig {
        /// The route config.
        #[prost(message, optional, tag = "1")]
        pub route_config: ::core::option::Option<
            super::super::super::super::google::protobuf::Any,
        >,
        /// The timestamp when the Route was last updated.
        #[prost(message, optional, tag = "2")]
        pub last_updated: ::core::option::Option<
            super::super::super::super::google::protobuf::Timestamp,
        >,
    }
    /// \[\#next-free-field: 6\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DynamicRouteConfig {
        /// This is the per-resource version information. This version is currently taken from the
        /// :ref:`version_info <envoy_v3_api_field_service.discovery.v3.DiscoveryResponse.version_info>` field at the time that
        /// the route configuration was loaded.
        #[prost(string, tag = "1")]
        pub version_info: ::prost::alloc::string::String,
        /// The route config.
        #[prost(message, optional, tag = "2")]
        pub route_config: ::core::option::Option<
            super::super::super::super::google::protobuf::Any,
        >,
        /// The timestamp when the Route was last updated.
        #[prost(message, optional, tag = "3")]
        pub last_updated: ::core::option::Option<
            super::super::super::super::google::protobuf::Timestamp,
        >,
        /// Set if the last update failed, cleared after the next successful update.
        /// The `error_state` field contains the rejected version of this particular
        /// resource along with the reason and timestamp. For successfully updated or
        /// acknowledged resource, this field should be empty.
        /// \[\#not-implemented-hide:\]
        #[prost(message, optional, tag = "4")]
        pub error_state: ::core::option::Option<super::UpdateFailureState>,
        /// The client status of this resource.
        /// \[\#not-implemented-hide:\]
        #[prost(enumeration = "super::ClientResourceStatus", tag = "5")]
        pub client_status: i32,
    }
}
/// Envoy's scoped RDS implementation fills this message with all currently loaded route
/// configuration scopes (defined via ScopedRouteConfigurationsSet protos). This message lists both
/// the scopes defined inline with the higher order object (i.e., the HttpConnectionManager) and the
/// dynamically obtained scopes via the SRDS API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScopedRoutesConfigDump {
    /// The statically loaded scoped route configs.
    #[prost(message, repeated, tag = "1")]
    pub inline_scoped_route_configs: ::prost::alloc::vec::Vec<
        scoped_routes_config_dump::InlineScopedRouteConfigs,
    >,
    /// The dynamically loaded scoped route configs.
    #[prost(message, repeated, tag = "2")]
    pub dynamic_scoped_route_configs: ::prost::alloc::vec::Vec<
        scoped_routes_config_dump::DynamicScopedRouteConfigs,
    >,
}
/// Nested message and enum types in `ScopedRoutesConfigDump`.
pub mod scoped_routes_config_dump {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InlineScopedRouteConfigs {
        /// The name assigned to the scoped route configurations.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// The scoped route configurations.
        #[prost(message, repeated, tag = "2")]
        pub scoped_route_configs: ::prost::alloc::vec::Vec<
            super::super::super::super::google::protobuf::Any,
        >,
        /// The timestamp when the scoped route config set was last updated.
        #[prost(message, optional, tag = "3")]
        pub last_updated: ::core::option::Option<
            super::super::super::super::google::protobuf::Timestamp,
        >,
    }
    /// \[\#next-free-field: 7\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DynamicScopedRouteConfigs {
        /// The name assigned to the scoped route configurations.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// This is the per-resource version information. This version is currently taken from the
        /// :ref:`version_info <envoy_v3_api_field_service.discovery.v3.DiscoveryResponse.version_info>` field at the time that
        /// the scoped routes configuration was loaded.
        #[prost(string, tag = "2")]
        pub version_info: ::prost::alloc::string::String,
        /// The scoped route configurations.
        #[prost(message, repeated, tag = "3")]
        pub scoped_route_configs: ::prost::alloc::vec::Vec<
            super::super::super::super::google::protobuf::Any,
        >,
        /// The timestamp when the scoped route config set was last updated.
        #[prost(message, optional, tag = "4")]
        pub last_updated: ::core::option::Option<
            super::super::super::super::google::protobuf::Timestamp,
        >,
        /// Set if the last update failed, cleared after the next successful update.
        /// The `error_state` field contains the rejected version of this particular
        /// resource along with the reason and timestamp. For successfully updated or
        /// acknowledged resource, this field should be empty.
        /// \[\#not-implemented-hide:\]
        #[prost(message, optional, tag = "5")]
        pub error_state: ::core::option::Option<super::UpdateFailureState>,
        /// The client status of this resource.
        /// \[\#not-implemented-hide:\]
        #[prost(enumeration = "super::ClientResourceStatus", tag = "6")]
        pub client_status: i32,
    }
}
/// Envoy's admin fill this message with all currently known endpoints. Endpoint
/// configuration information can be used to recreate an Envoy configuration by populating all
/// endpoints as static endpoints or by returning them in an EDS response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndpointsConfigDump {
    /// The statically loaded endpoint configs.
    #[prost(message, repeated, tag = "2")]
    pub static_endpoint_configs: ::prost::alloc::vec::Vec<
        endpoints_config_dump::StaticEndpointConfig,
    >,
    /// The dynamically loaded endpoint configs.
    #[prost(message, repeated, tag = "3")]
    pub dynamic_endpoint_configs: ::prost::alloc::vec::Vec<
        endpoints_config_dump::DynamicEndpointConfig,
    >,
}
/// Nested message and enum types in `EndpointsConfigDump`.
pub mod endpoints_config_dump {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StaticEndpointConfig {
        /// The endpoint config.
        #[prost(message, optional, tag = "1")]
        pub endpoint_config: ::core::option::Option<
            super::super::super::super::google::protobuf::Any,
        >,
        /// \[\#not-implemented-hide:\] The timestamp when the Endpoint was last updated.
        #[prost(message, optional, tag = "2")]
        pub last_updated: ::core::option::Option<
            super::super::super::super::google::protobuf::Timestamp,
        >,
    }
    /// \[\#next-free-field: 6\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DynamicEndpointConfig {
        /// \[\#not-implemented-hide:\] This is the per-resource version information. This version is currently taken from the
        /// :ref:`version_info <envoy_v3_api_field_service.discovery.v3.DiscoveryResponse.version_info>` field at the time that
        /// the endpoint configuration was loaded.
        #[prost(string, tag = "1")]
        pub version_info: ::prost::alloc::string::String,
        /// The endpoint config.
        #[prost(message, optional, tag = "2")]
        pub endpoint_config: ::core::option::Option<
            super::super::super::super::google::protobuf::Any,
        >,
        /// \[\#not-implemented-hide:\] The timestamp when the Endpoint was last updated.
        #[prost(message, optional, tag = "3")]
        pub last_updated: ::core::option::Option<
            super::super::super::super::google::protobuf::Timestamp,
        >,
        /// Set if the last update failed, cleared after the next successful update.
        /// The `error_state` field contains the rejected version of this particular
        /// resource along with the reason and timestamp. For successfully updated or
        /// acknowledged resource, this field should be empty.
        /// \[\#not-implemented-hide:\]
        #[prost(message, optional, tag = "4")]
        pub error_state: ::core::option::Option<super::UpdateFailureState>,
        /// The client status of this resource.
        /// \[\#not-implemented-hide:\]
        #[prost(enumeration = "super::ClientResourceStatus", tag = "5")]
        pub client_status: i32,
    }
}
/// Envoy's ECDS service fills this message with all currently extension
/// configuration. Extension configuration information can be used to recreate
/// an Envoy ECDS listener and HTTP filters as static filters or by returning
/// them in ECDS response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EcdsConfigDump {
    /// The ECDS filter configs.
    #[prost(message, repeated, tag = "1")]
    pub ecds_filters: ::prost::alloc::vec::Vec<ecds_config_dump::EcdsFilterConfig>,
}
/// Nested message and enum types in `EcdsConfigDump`.
pub mod ecds_config_dump {
    /// \[\#next-free-field: 6\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EcdsFilterConfig {
        /// This is the per-resource version information. This version is currently
        /// taken from the :ref:`version_info <envoy_v3_api_field_service.discovery.v3.DiscoveryResponse.version_info>`
        /// field at the time that the ECDS filter was loaded.
        #[prost(string, tag = "1")]
        pub version_info: ::prost::alloc::string::String,
        /// The ECDS filter config.
        #[prost(message, optional, tag = "2")]
        pub ecds_filter: ::core::option::Option<
            super::super::super::super::google::protobuf::Any,
        >,
        /// The timestamp when the ECDS filter was last updated.
        #[prost(message, optional, tag = "3")]
        pub last_updated: ::core::option::Option<
            super::super::super::super::google::protobuf::Timestamp,
        >,
        /// Set if the last update failed, cleared after the next successful update.
        /// The `error_state` field contains the rejected version of this
        /// particular resource along with the reason and timestamp. For successfully
        /// updated or acknowledged resource, this field should be empty.
        /// \[\#not-implemented-hide:\]
        #[prost(message, optional, tag = "4")]
        pub error_state: ::core::option::Option<super::UpdateFailureState>,
        /// The client status of this resource.
        /// \[\#not-implemented-hide:\]
        #[prost(enumeration = "super::ClientResourceStatus", tag = "5")]
        pub client_status: i32,
    }
}
/// Resource status from the view of a xDS client, which tells the synchronization
/// status between the xDS client and the xDS server.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClientResourceStatus {
    /// Resource status is not available/unknown.
    Unknown = 0,
    /// Client requested this resource but hasn't received any update from management
    /// server. The client will not fail requests, but will queue them until update
    /// arrives or the client times out waiting for the resource.
    Requested = 1,
    /// This resource has been requested by the client but has either not been
    /// delivered by the server or was previously delivered by the server and then
    /// subsequently removed from resources provided by the server. For more
    /// information, please refer to the :ref:`"Knowing When a Requested Resource Does Not Exist" <xds_protocol_resource_not_existed>` section.
    DoesNotExist = 2,
    /// Client received this resource and replied with ACK.
    Acked = 3,
    /// Client received this resource and replied with NACK.
    Nacked = 4,
}
impl ClientResourceStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ClientResourceStatus::Unknown => "UNKNOWN",
            ClientResourceStatus::Requested => "REQUESTED",
            ClientResourceStatus::DoesNotExist => "DOES_NOT_EXIST",
            ClientResourceStatus::Acked => "ACKED",
            ClientResourceStatus::Nacked => "NACKED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "REQUESTED" => Some(Self::Requested),
            "DOES_NOT_EXIST" => Some(Self::DoesNotExist),
            "ACKED" => Some(Self::Acked),
            "NACKED" => Some(Self::Nacked),
            _ => None,
        }
    }
}
/// The :ref:`/config_dump <operations_admin_interface_config_dump>` admin endpoint uses this wrapper
/// message to maintain and serve arbitrary configuration information from any component in Envoy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigDump {
    /// This list is serialized and dumped in its entirety at the
    /// :ref:`/config_dump <operations_admin_interface_config_dump>` endpoint.
    ///
    /// The following configurations are currently supported and will be dumped in the order given
    /// below:
    ///
    /// * `bootstrap`: :ref:`BootstrapConfigDump <envoy_v3_api_msg_admin.v3.BootstrapConfigDump>`
    /// * `clusters`: :ref:`ClustersConfigDump <envoy_v3_api_msg_admin.v3.ClustersConfigDump>`
    /// * `ecds_filter_http`: :ref:`EcdsConfigDump <envoy_v3_api_msg_admin.v3.EcdsConfigDump>`
    /// * `ecds_filter_quic_listener`: :ref:`EcdsConfigDump <envoy_v3_api_msg_admin.v3.EcdsConfigDump>`
    /// * `ecds_filter_tcp_listener`: :ref:`EcdsConfigDump <envoy_v3_api_msg_admin.v3.EcdsConfigDump>`
    /// * `endpoints`:  :ref:`EndpointsConfigDump <envoy_v3_api_msg_admin.v3.EndpointsConfigDump>`
    /// * `listeners`: :ref:`ListenersConfigDump <envoy_v3_api_msg_admin.v3.ListenersConfigDump>`
    /// * `scoped_routes`: :ref:`ScopedRoutesConfigDump <envoy_v3_api_msg_admin.v3.ScopedRoutesConfigDump>`
    /// * `routes`:  :ref:`RoutesConfigDump <envoy_v3_api_msg_admin.v3.RoutesConfigDump>`
    /// * `secrets`:  :ref:`SecretsConfigDump <envoy_v3_api_msg_admin.v3.SecretsConfigDump>`
    ///
    /// EDS Configuration will only be dumped by using parameter `?include_eds`
    ///
    /// Currently ECDS is supported in HTTP and listener filters. Note, ECDS configuration for
    /// either HTTP or listener filter will only be dumped if it is actually configured.
    ///
    /// You can filter output with the resource and mask query parameters.
    /// See :ref:`/config_dump?resource={} <operations_admin_interface_config_dump_by_resource>`,
    /// :ref:`/config_dump?mask={} <operations_admin_interface_config_dump_by_mask>`,
    /// or :ref:`/config_dump?resource={},mask={} <operations_admin_interface_config_dump_by_resource_and_mask>` for more information.
    #[prost(message, repeated, tag = "1")]
    pub configs: ::prost::alloc::vec::Vec<super::super::super::google::protobuf::Any>,
}
/// This message describes the bootstrap configuration that Envoy was started with. This includes
/// any CLI overrides that were merged. Bootstrap configuration information can be used to recreate
/// the static portions of an Envoy configuration by reusing the output as the bootstrap
/// configuration for another Envoy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BootstrapConfigDump {
    #[prost(message, optional, tag = "1")]
    pub bootstrap: ::core::option::Option<
        super::super::config::bootstrap::v3::Bootstrap,
    >,
    /// The timestamp when the BootstrapConfig was last updated.
    #[prost(message, optional, tag = "2")]
    pub last_updated: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
}
/// Envoys SDS implementation fills this message with all secrets fetched dynamically via SDS.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretsConfigDump {
    /// The statically loaded secrets.
    #[prost(message, repeated, tag = "1")]
    pub static_secrets: ::prost::alloc::vec::Vec<secrets_config_dump::StaticSecret>,
    /// The dynamically loaded active secrets. These are secrets that are available to service
    /// clusters or listeners.
    #[prost(message, repeated, tag = "2")]
    pub dynamic_active_secrets: ::prost::alloc::vec::Vec<
        secrets_config_dump::DynamicSecret,
    >,
    /// The dynamically loaded warming secrets. These are secrets that are currently undergoing
    /// warming in preparation to service clusters or listeners.
    #[prost(message, repeated, tag = "3")]
    pub dynamic_warming_secrets: ::prost::alloc::vec::Vec<
        secrets_config_dump::DynamicSecret,
    >,
}
/// Nested message and enum types in `SecretsConfigDump`.
pub mod secrets_config_dump {
    /// DynamicSecret contains secret information fetched via SDS.
    /// \[\#next-free-field: 7\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DynamicSecret {
        /// The name assigned to the secret.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// This is the per-resource version information.
        #[prost(string, tag = "2")]
        pub version_info: ::prost::alloc::string::String,
        /// The timestamp when the secret was last updated.
        #[prost(message, optional, tag = "3")]
        pub last_updated: ::core::option::Option<
            super::super::super::super::google::protobuf::Timestamp,
        >,
        /// The actual secret information.
        /// Security sensitive information is redacted (replaced with "\[redacted\]") for
        /// private keys and passwords in TLS certificates.
        #[prost(message, optional, tag = "4")]
        pub secret: ::core::option::Option<
            super::super::super::super::google::protobuf::Any,
        >,
        /// Set if the last update failed, cleared after the next successful update.
        /// The *error_state* field contains the rejected version of this particular
        /// resource along with the reason and timestamp. For successfully updated or
        /// acknowledged resource, this field should be empty.
        /// \[\#not-implemented-hide:\]
        #[prost(message, optional, tag = "5")]
        pub error_state: ::core::option::Option<super::UpdateFailureState>,
        /// The client status of this resource.
        /// \[\#not-implemented-hide:\]
        #[prost(enumeration = "super::ClientResourceStatus", tag = "6")]
        pub client_status: i32,
    }
    /// StaticSecret specifies statically loaded secret in bootstrap.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StaticSecret {
        /// The name assigned to the secret.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// The timestamp when the secret was last updated.
        #[prost(message, optional, tag = "2")]
        pub last_updated: ::core::option::Option<
            super::super::super::super::google::protobuf::Timestamp,
        >,
        /// The actual secret information.
        /// Security sensitive information is redacted (replaced with "\[redacted\]") for
        /// private keys and passwords in TLS certificates.
        #[prost(message, optional, tag = "3")]
        pub secret: ::core::option::Option<
            super::super::super::super::google::protobuf::Any,
        >,
    }
}
/// Dumps of unready targets of envoy init managers. Envoy's admin fills this message with init managers,
/// which provides the information of their unready targets.
/// The :ref:`/init_dump <operations_admin_interface_init_dump>` will dump all unready targets information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnreadyTargetsDumps {
    /// You can choose specific component to dump unready targets with mask query parameter.
    /// See :ref:`/init_dump?mask={} <operations_admin_interface_init_dump_by_mask>` for more information.
    /// The dumps of unready targets of all init managers.
    #[prost(message, repeated, tag = "1")]
    pub unready_targets_dumps: ::prost::alloc::vec::Vec<
        unready_targets_dumps::UnreadyTargetsDump,
    >,
}
/// Nested message and enum types in `UnreadyTargetsDumps`.
pub mod unready_targets_dumps {
    /// Message of unready targets information of an init manager.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UnreadyTargetsDump {
        /// Name of the init manager. Example: "init_manager_xxx".
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Names of unready targets of the init manager. Example: "target_xxx".
        #[prost(string, repeated, tag = "2")]
        pub target_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// Admin endpoint uses this wrapper for `/listeners` to display listener status information.
/// See :ref:`/listeners <operations_admin_interface_listeners>` for more information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Listeners {
    /// List of listener statuses.
    #[prost(message, repeated, tag = "1")]
    pub listener_statuses: ::prost::alloc::vec::Vec<ListenerStatus>,
}
/// Details an individual listener's current status.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenerStatus {
    /// Name of the listener
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The actual local address that the listener is listening on. If a listener was configured
    /// to listen on port 0, then this address has the port that was allocated by the OS.
    #[prost(message, optional, tag = "2")]
    pub local_address: ::core::option::Option<super::super::config::core::v3::Address>,
    /// The additional addresses the listener is listening on as specified via the :ref:`additional_addresses <envoy_v3_api_field_config.listener.v3.Listener.additional_addresses>`
    /// configuration.
    #[prost(message, repeated, tag = "3")]
    pub additional_local_addresses: ::prost::alloc::vec::Vec<
        super::super::config::core::v3::Address,
    >,
}
/// Proto representation of the internal memory consumption of an Envoy instance. These represent
/// values extracted from an internal TCMalloc instance. For more information, see the section of the
/// docs entitled ["Generic Tcmalloc Status"](<https://gperftools.github.io/gperftools/tcmalloc.html>).
/// \[\#next-free-field: 7\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Memory {
    /// The number of bytes allocated by the heap for Envoy. This is an alias for
    /// `generic.current_allocated_bytes`.
    #[prost(uint64, tag = "1")]
    pub allocated: u64,
    /// The number of bytes reserved by the heap but not necessarily allocated. This is an alias for
    /// `generic.heap_size`.
    #[prost(uint64, tag = "2")]
    pub heap_size: u64,
    /// The number of bytes in free, unmapped pages in the page heap. These bytes always count towards
    /// virtual memory usage, and depending on the OS, typically do not count towards physical memory
    /// usage. This is an alias for `tcmalloc.pageheap_unmapped_bytes`.
    #[prost(uint64, tag = "3")]
    pub pageheap_unmapped: u64,
    /// The number of bytes in free, mapped pages in the page heap. These bytes always count towards
    /// virtual memory usage, and unless the underlying memory is swapped out by the OS, they also
    /// count towards physical memory usage. This is an alias for `tcmalloc.pageheap_free_bytes`.
    #[prost(uint64, tag = "4")]
    pub pageheap_free: u64,
    /// The amount of memory used by the TCMalloc thread caches (for small objects). This is an alias
    /// for `tcmalloc.current_total_thread_cache_bytes`.
    #[prost(uint64, tag = "5")]
    pub total_thread_cache: u64,
    /// The number of bytes of the physical memory usage by the allocator. This is an alias for
    /// `generic.total_physical_bytes`.
    #[prost(uint64, tag = "6")]
    pub total_physical_bytes: u64,
}
/// Proto representation of the statistics collected upon absl::Mutex contention, if Envoy is run
/// under :option:`--enable-mutex-tracing`. For more information, see the `absl::Mutex`
/// [docs](<https://abseil.io/about/design/mutex#extra-features>).
///
/// *NB*: The wait cycles below are measured by `absl::base_internal::CycleClock`, and may not
/// correspond to core clock frequency. For more information, see the `CycleClock`
/// [docs](<https://github.com/abseil/abseil-cpp/blob/master/absl/base/internal/cycleclock.h>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutexStats {
    /// The number of individual mutex contentions which have occurred since startup.
    #[prost(uint64, tag = "1")]
    pub num_contentions: u64,
    /// The length of the current contention wait cycle.
    #[prost(uint64, tag = "2")]
    pub current_wait_cycles: u64,
    /// The lifetime total of all contention wait cycles.
    #[prost(uint64, tag = "3")]
    pub lifetime_wait_cycles: u64,
}
/// Proto representation of the value returned by /server_info, containing
/// server version/server status information.
/// \[\#next-free-field: 8\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerInfo {
    /// Server version.
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    /// State of the server.
    #[prost(enumeration = "server_info::State", tag = "2")]
    pub state: i32,
    /// Uptime since current epoch was started.
    #[prost(message, optional, tag = "3")]
    pub uptime_current_epoch: ::core::option::Option<
        super::super::super::google::protobuf::Duration,
    >,
    /// Uptime since the start of the first epoch.
    #[prost(message, optional, tag = "4")]
    pub uptime_all_epochs: ::core::option::Option<
        super::super::super::google::protobuf::Duration,
    >,
    /// Hot restart version.
    #[prost(string, tag = "5")]
    pub hot_restart_version: ::prost::alloc::string::String,
    /// Command line options the server is currently running with.
    #[prost(message, optional, tag = "6")]
    pub command_line_options: ::core::option::Option<CommandLineOptions>,
    /// Populated node identity of this server.
    #[prost(message, optional, tag = "7")]
    pub node: ::core::option::Option<super::super::config::core::v3::Node>,
}
/// Nested message and enum types in `ServerInfo`.
pub mod server_info {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// Server is live and serving traffic.
        Live = 0,
        /// Server is draining listeners in response to external health checks failing.
        Draining = 1,
        /// Server has not yet completed cluster manager initialization.
        PreInitializing = 2,
        /// Server is running the cluster manager initialization callbacks (e.g., RDS).
        Initializing = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Live => "LIVE",
                State::Draining => "DRAINING",
                State::PreInitializing => "PRE_INITIALIZING",
                State::Initializing => "INITIALIZING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LIVE" => Some(Self::Live),
                "DRAINING" => Some(Self::Draining),
                "PRE_INITIALIZING" => Some(Self::PreInitializing),
                "INITIALIZING" => Some(Self::Initializing),
                _ => None,
            }
        }
    }
}
/// \[\#next-free-field: 39\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandLineOptions {
    /// See :option:`--base-id` for details.
    #[prost(uint64, tag = "1")]
    pub base_id: u64,
    /// See :option:`--use-dynamic-base-id` for details.
    #[prost(bool, tag = "31")]
    pub use_dynamic_base_id: bool,
    /// See :option:`--base-id-path` for details.
    #[prost(string, tag = "32")]
    pub base_id_path: ::prost::alloc::string::String,
    /// See :option:`--concurrency` for details.
    #[prost(uint32, tag = "2")]
    pub concurrency: u32,
    /// See :option:`--config-path` for details.
    #[prost(string, tag = "3")]
    pub config_path: ::prost::alloc::string::String,
    /// See :option:`--config-yaml` for details.
    #[prost(string, tag = "4")]
    pub config_yaml: ::prost::alloc::string::String,
    /// See :option:`--allow-unknown-static-fields` for details.
    #[prost(bool, tag = "5")]
    pub allow_unknown_static_fields: bool,
    /// See :option:`--reject-unknown-dynamic-fields` for details.
    #[prost(bool, tag = "26")]
    pub reject_unknown_dynamic_fields: bool,
    /// See :option:`--ignore-unknown-dynamic-fields` for details.
    #[prost(bool, tag = "30")]
    pub ignore_unknown_dynamic_fields: bool,
    /// See :option:`--admin-address-path` for details.
    #[prost(string, tag = "6")]
    pub admin_address_path: ::prost::alloc::string::String,
    /// See :option:`--local-address-ip-version` for details.
    #[prost(enumeration = "command_line_options::IpVersion", tag = "7")]
    pub local_address_ip_version: i32,
    /// See :option:`--log-level` for details.
    #[prost(string, tag = "8")]
    pub log_level: ::prost::alloc::string::String,
    /// See :option:`--component-log-level` for details.
    #[prost(string, tag = "9")]
    pub component_log_level: ::prost::alloc::string::String,
    /// See :option:`--log-format` for details.
    #[prost(string, tag = "10")]
    pub log_format: ::prost::alloc::string::String,
    /// See :option:`--log-format-escaped` for details.
    #[prost(bool, tag = "27")]
    pub log_format_escaped: bool,
    /// See :option:`--log-path` for details.
    #[prost(string, tag = "11")]
    pub log_path: ::prost::alloc::string::String,
    /// See :option:`--service-cluster` for details.
    #[prost(string, tag = "13")]
    pub service_cluster: ::prost::alloc::string::String,
    /// See :option:`--service-node` for details.
    #[prost(string, tag = "14")]
    pub service_node: ::prost::alloc::string::String,
    /// See :option:`--service-zone` for details.
    #[prost(string, tag = "15")]
    pub service_zone: ::prost::alloc::string::String,
    /// See :option:`--file-flush-interval-msec` for details.
    #[prost(message, optional, tag = "16")]
    pub file_flush_interval: ::core::option::Option<
        super::super::super::google::protobuf::Duration,
    >,
    /// See :option:`--drain-time-s` for details.
    #[prost(message, optional, tag = "17")]
    pub drain_time: ::core::option::Option<
        super::super::super::google::protobuf::Duration,
    >,
    /// See :option:`--drain-strategy` for details.
    #[prost(enumeration = "command_line_options::DrainStrategy", tag = "33")]
    pub drain_strategy: i32,
    /// See :option:`--parent-shutdown-time-s` for details.
    #[prost(message, optional, tag = "18")]
    pub parent_shutdown_time: ::core::option::Option<
        super::super::super::google::protobuf::Duration,
    >,
    /// See :option:`--mode` for details.
    #[prost(enumeration = "command_line_options::Mode", tag = "19")]
    pub mode: i32,
    /// See :option:`--disable-hot-restart` for details.
    #[prost(bool, tag = "22")]
    pub disable_hot_restart: bool,
    /// See :option:`--enable-mutex-tracing` for details.
    #[prost(bool, tag = "23")]
    pub enable_mutex_tracing: bool,
    /// See :option:`--restart-epoch` for details.
    #[prost(uint32, tag = "24")]
    pub restart_epoch: u32,
    /// See :option:`--cpuset-threads` for details.
    #[prost(bool, tag = "25")]
    pub cpuset_threads: bool,
    /// See :option:`--disable-extensions` for details.
    #[prost(string, repeated, tag = "28")]
    pub disabled_extensions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// See :option:`--enable-fine-grain-logging` for details.
    #[prost(bool, tag = "34")]
    pub enable_fine_grain_logging: bool,
    /// See :option:`--socket-path` for details.
    #[prost(string, tag = "35")]
    pub socket_path: ::prost::alloc::string::String,
    /// See :option:`--socket-mode` for details.
    #[prost(uint32, tag = "36")]
    pub socket_mode: u32,
    /// See :option:`--enable-core-dump` for details.
    #[prost(bool, tag = "37")]
    pub enable_core_dump: bool,
    /// See :option:`--stats-tag` for details.
    #[prost(string, repeated, tag = "38")]
    pub stats_tag: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `CommandLineOptions`.
pub mod command_line_options {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum IpVersion {
        V4 = 0,
        V6 = 1,
    }
    impl IpVersion {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IpVersion::V4 => "v4",
                IpVersion::V6 => "v6",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "v4" => Some(Self::V4),
                "v6" => Some(Self::V6),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Mode {
        /// Validate configs and then serve traffic normally.
        Serve = 0,
        /// Validate configs and exit.
        Validate = 1,
        /// Completely load and initialize the config, and then exit without running the listener loop.
        InitOnly = 2,
    }
    impl Mode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Mode::Serve => "Serve",
                Mode::Validate => "Validate",
                Mode::InitOnly => "InitOnly",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "Serve" => Some(Self::Serve),
                "Validate" => Some(Self::Validate),
                "InitOnly" => Some(Self::InitOnly),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DrainStrategy {
        /// Gradually discourage connections over the course of the drain period.
        Gradual = 0,
        /// Discourage all connections for the duration of the drain sequence.
        Immediate = 1,
    }
    impl DrainStrategy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DrainStrategy::Gradual => "Gradual",
                DrainStrategy::Immediate => "Immediate",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "Gradual" => Some(Self::Gradual),
                "Immediate" => Some(Self::Immediate),
                _ => None,
            }
        }
    }
}
/// The /tap admin request body that is used to configure an active tap session.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TapRequest {
    /// The opaque configuration ID used to match the configuration to a loaded extension.
    /// A tap extension configures a similar opaque ID that is used to match.
    #[prost(string, tag = "1")]
    pub config_id: ::prost::alloc::string::String,
    /// The tap configuration to load.
    #[prost(message, optional, tag = "2")]
    pub tap_config: ::core::option::Option<super::super::config::tap::v3::TapConfig>,
}
