/// Configuration for the filter state based dynamic forward proxy filter. See the
/// :ref:`architecture overview <arch_overview_http_dynamic_forward_proxy>` for
/// more information. Note this filter must be used in conjunction to another filter that
/// sets the 'envoy.upstream.dynamic_host' and the 'envoy.upstream.dynamic_port' filter
/// state keys for the required upstream UDP session.
/// \[\#extension: envoy.filters.udp.session.dynamic_forward_proxy\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterConfig {
    /// The prefix to use when emitting :ref:`statistics <config_udp_session_filters_dynamic_forward_proxy_stats>`.
    #[prost(string, tag = "1")]
    pub stat_prefix: ::prost::alloc::string::String,
    /// If configured, the filter will buffer datagrams in case that it is waiting for a DNS response.
    /// If this field is not configured, there will be no buffering and downstream datagrams that arrive
    /// while the DNS resolution is in progress will be dropped. In case this field is set but the options
    /// are not configured, the default values will be applied as described in the `BufferOptions`.
    #[prost(message, optional, tag = "3")]
    pub buffer_options: ::core::option::Option<filter_config::BufferOptions>,
    #[prost(oneof = "filter_config::ImplementationSpecifier", tags = "2")]
    pub implementation_specifier: ::core::option::Option<
        filter_config::ImplementationSpecifier,
    >,
}
/// Nested message and enum types in `FilterConfig`.
pub mod filter_config {
    /// Configuration for UDP datagrams buffering.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BufferOptions {
        /// If set, the filter will only buffer datagrams up to the requested limit, and will drop
        /// new UDP datagrams if the buffer contains the max_buffered_datagrams value at the time
        /// of a new datagram arrival. If not set, the default value is 1024 datagrams.
        #[prost(message, optional, tag = "1")]
        pub max_buffered_datagrams: ::core::option::Option<
            super::super::super::super::super::super::super::super::super::google::protobuf::UInt32Value,
        >,
        /// If set, the filter will only buffer datagrams up to the requested total buffered bytes limit,
        /// and will drop new UDP datagrams if the buffer contains the max_buffered_datagrams value
        /// at the time of a new datagram arrival. If not set, the default value is 16,384 (16KB).
        #[prost(message, optional, tag = "2")]
        pub max_buffered_bytes: ::core::option::Option<
            super::super::super::super::super::super::super::super::super::google::protobuf::UInt64Value,
        >,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ImplementationSpecifier {
        /// The DNS cache configuration that the filter will attach to. Note this
        /// configuration must match that of associated :ref:`dynamic forward proxy cluster configuration <envoy_v3_api_field_extensions.clusters.dynamic_forward_proxy.v3.ClusterConfig.dns_cache_config>`.
        #[prost(message, tag = "2")]
        DnsCacheConfig(
            super::super::super::super::super::super::super::common::dynamic_forward_proxy::v3::DnsCacheConfig,
        ),
    }
}
