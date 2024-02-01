/// \[\#next-free-field: 10\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisProxy {
    /// The prefix to use when emitting :ref:`statistics <config_network_filters_redis_proxy_stats>`.
    #[prost(string, tag = "1")]
    pub stat_prefix: ::prost::alloc::string::String,
    /// Network settings for the connection pool to the upstream clusters.
    #[prost(message, optional, tag = "3")]
    pub settings: ::core::option::Option<redis_proxy::ConnPoolSettings>,
    /// Indicates that latency stat should be computed in microseconds. By default it is computed in
    /// milliseconds. This does not apply to upstream command stats currently.
    #[prost(bool, tag = "4")]
    pub latency_in_micros: bool,
    /// List of **unique** prefixes used to separate keys from different workloads to different
    /// clusters. Envoy will always favor the longest match first in case of overlap. A catch-all
    /// cluster can be used to forward commands when there is no match. Time complexity of the
    /// lookups are in O(min(longest key prefix, key length)).
    ///
    /// Example:
    ///
    /// .. code-block:: yaml
    ///
    /// ```text
    /// prefix_routes:
    ///   routes:
    ///     - prefix: "ab"
    ///       cluster: "cluster_a"
    ///     - prefix: "abc"
    ///       cluster: "cluster_b"
    /// ```
    ///
    /// When using the above routes, the following prefixes would be sent to:
    ///
    /// * `get abc:users` would retrieve the key 'abc:users' from cluster_b.
    /// * `get ab:users` would retrieve the key 'ab:users' from cluster_a.
    /// * `get z:users` would return a NoUpstreamHost error. A :ref:`catch-all route<envoy_v3_api_field_extensions.filters.network.redis_proxy.v3.RedisProxy.PrefixRoutes.catch_all_route>`
    ///   would have retrieved the key from that cluster instead.
    ///
    /// See the :ref:`configuration section <arch_overview_redis_configuration>` of the architecture overview for recommendations on
    /// configuring the backing clusters.
    #[prost(message, optional, tag = "5")]
    pub prefix_routes: ::core::option::Option<redis_proxy::PrefixRoutes>,
    /// Authenticate Redis client connections locally by forcing downstream clients to issue a `Redis AUTH command <<https://redis.io/commands/auth>`\_> with this password before enabling any other
    /// command. If an AUTH command's password matches this password, an "OK" response will be returned
    /// to the client. If the AUTH command password does not match this password, then an "ERR invalid
    /// password" error will be returned. If any other command is received before AUTH when this
    /// password is set, then a "NOAUTH Authentication required." error response will be sent to the
    /// client. If an AUTH command is received when the password is not set, then an "ERR Client sent
    /// AUTH, but no password is set" error will be returned.
    ///
    /// .. attention::
    /// This field is deprecated. Use :ref:`downstream_auth_passwords <envoy_v3_api_field_extensions.filters.network.redis_proxy.v3.RedisProxy.downstream_auth_passwords>`.
    #[deprecated]
    #[prost(message, optional, tag = "6")]
    pub downstream_auth_password: ::core::option::Option<
        super::super::super::super::super::config::core::v3::DataSource,
    >,
    /// Authenticate Redis client connections locally by forcing downstream clients to issue a `Redis AUTH command <<https://redis.io/commands/auth>`\_> with one of these passwords before enabling any other
    /// command. If an AUTH command's password matches one of these passwords, an "OK" response will be returned
    /// to the client. If the AUTH command password does not match, then an "ERR invalid
    /// password" error will be returned. If any other command is received before AUTH when the
    /// password(s) are set, then a "NOAUTH Authentication required." error response will be sent to the
    /// client. If an AUTH command is received when the password is not set, then an "ERR Client sent
    /// AUTH, but no password is set" error will be returned.
    #[prost(message, repeated, tag = "9")]
    pub downstream_auth_passwords: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::core::v3::DataSource,
    >,
    /// List of faults to inject. Faults currently come in two flavors:
    ///
    /// * Delay, which delays a request.
    /// * Error, which responds to a request with an error. Errors can also have delays attached.
    ///
    /// Example:
    ///
    /// .. code-block:: yaml
    ///
    /// ```text
    /// faults:
    /// - fault_type: ERROR
    ///   fault_enabled:
    ///     default_value:
    ///       numerator: 10
    ///       denominator: HUNDRED
    ///     runtime_key: "bogus_key"
    ///     commands:
    ///     - GET
    ///   - fault_type: DELAY
    ///     fault_enabled:
    ///       default_value:
    ///         numerator: 10
    ///         denominator: HUNDRED
    ///       runtime_key: "bogus_key"
    ///     delay: 2s
    /// ```
    ///
    /// See the :ref:`fault injection section <config_network_filters_redis_proxy_fault_injection>` for more information on how to configure this.
    #[prost(message, repeated, tag = "8")]
    pub faults: ::prost::alloc::vec::Vec<redis_proxy::RedisFault>,
    /// If a username is provided an ACL style AUTH command will be required with a username and password.
    /// Authenticate Redis client connections locally by forcing downstream clients to issue a `Redis AUTH command <<https://redis.io/commands/auth>`\_> with this username and the `downstream_auth_password`
    /// before enabling any other command. If an AUTH command's username and password matches this username
    /// and the `downstream_auth_password` , an "OK" response will be returned to the client. If the AUTH
    /// command username or password does not match this username or the `downstream_auth_password`, then an
    /// "WRONGPASS invalid username-password pair" error will be returned. If any other command is received before AUTH when this
    /// password is set, then a "NOAUTH Authentication required." error response will be sent to the
    /// client. If an AUTH command is received when the password is not set, then an "ERR Client sent
    /// AUTH, but no ACL is set" error will be returned.
    #[prost(message, optional, tag = "7")]
    pub downstream_auth_username: ::core::option::Option<
        super::super::super::super::super::config::core::v3::DataSource,
    >,
}
/// Nested message and enum types in `RedisProxy`.
pub mod redis_proxy {
    /// Redis connection pool settings.
    /// \[\#next-free-field: 11\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConnPoolSettings {
        /// Per-operation timeout in milliseconds. The timer starts when the first
        /// command of a pipeline is written to the backend connection. Each response received from Redis
        /// resets the timer since it signifies that the next command is being processed by the backend.
        /// The only exception to this behavior is when a connection to a backend is not yet established.
        /// In that case, the connect timeout on the cluster will govern the timeout until the connection
        /// is ready.
        #[prost(message, optional, tag = "1")]
        pub op_timeout: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::Duration,
        >,
        /// Use hash tagging on every redis key to guarantee that keys with the same hash tag will be
        /// forwarded to the same upstream. The hash key used for determining the upstream in a
        /// consistent hash ring configuration will be computed from the hash tagged key instead of the
        /// whole key. The algorithm used to compute the hash tag is identical to the `redis-cluster implementation <<https://redis.io/topics/cluster-spec#keys-hash-tags>`\_.>
        ///
        /// Examples:
        ///
        /// * '{user1000}.following' and '{user1000}.followers' **will** be sent to the same upstream
        /// * '{user1000}.following' and '{user1001}.following' **might** be sent to the same upstream
        #[prost(bool, tag = "2")]
        pub enable_hashtagging: bool,
        /// Accept `moved and ask redirection <<https://redis.io/topics/cluster-spec#redirection-and-resharding>`\_> errors from upstream
        /// redis servers, and retry commands to the specified target server. The target server does not
        /// need to be known to the cluster manager. If the command cannot be redirected, then the
        /// original error is passed downstream unchanged. By default, this support is not enabled.
        #[prost(bool, tag = "3")]
        pub enable_redirection: bool,
        /// If `enable_redirection` is set to true this option configures the DNS cache that the
        /// connection pool will use to resolve hostnames that are returned with MOVED and ASK responses.
        /// If no configuration is provided, DNS lookups will not be performed (and thus the MOVED/ASK errors
        /// will be propagated verbatim to the user).
        #[prost(message, optional, tag = "9")]
        pub dns_cache_config: ::core::option::Option<
            super::super::super::super::super::common::dynamic_forward_proxy::v3::DnsCacheConfig,
        >,
        /// Maximum size of encoded request buffer before flush is triggered and encoded requests
        /// are sent upstream. If this is unset, the buffer flushes whenever it receives data
        /// and performs no batching.
        /// This feature makes it possible for multiple clients to send requests to Envoy and have
        /// them batched- for example if one is running several worker processes, each with its own
        /// Redis connection. There is no benefit to using this with a single downstream process.
        /// Recommended size (if enabled) is 1024 bytes.
        #[prost(uint32, tag = "4")]
        pub max_buffer_size_before_flush: u32,
        /// The encoded request buffer is flushed N milliseconds after the first request has been
        /// encoded, unless the buffer size has already exceeded `max_buffer_size_before_flush`.
        /// If `max_buffer_size_before_flush` is not set, this flush timer is not used. Otherwise,
        /// the timer should be set according to the number of clients, overall request rate and
        /// desired maximum latency for a single command. For example, if there are many requests
        /// being batched together at a high rate, the buffer will likely be filled before the timer
        /// fires. Alternatively, if the request rate is lower the buffer will not be filled as often
        /// before the timer fires.
        /// If `max_buffer_size_before_flush` is set, but `buffer_flush_timeout` is not, the latter
        /// defaults to 3ms.
        #[prost(message, optional, tag = "5")]
        pub buffer_flush_timeout: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::Duration,
        >,
        /// `max_upstream_unknown_connections` controls how many upstream connections to unknown hosts
        /// can be created at any given time by any given worker thread (see `enable_redirection` for
        /// more details). If the host is unknown and a connection cannot be created due to enforcing
        /// this limit, then redirection will fail and the original redirection error will be passed
        /// downstream unchanged. This limit defaults to 100.
        #[prost(message, optional, tag = "6")]
        pub max_upstream_unknown_connections: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::UInt32Value,
        >,
        /// Enable per-command statistics per upstream cluster, in addition to the filter level aggregate
        /// count. These commands are measured in microseconds.
        #[prost(bool, tag = "8")]
        pub enable_command_stats: bool,
        /// Read policy. The default is to read from the primary.
        #[prost(enumeration = "conn_pool_settings::ReadPolicy", tag = "7")]
        pub read_policy: i32,
        /// Ops or connection timeout triggers reconnection to redis server which could result in reconnection
        /// storm to busy redis server. This config is a protection to rate limit reconnection rate.
        /// If not set, there will be no rate limiting on the reconnection.
        #[prost(message, optional, tag = "10")]
        pub connection_rate_limit: ::core::option::Option<ConnectionRateLimit>,
    }
    /// Nested message and enum types in `ConnPoolSettings`.
    pub mod conn_pool_settings {
        /// ReadPolicy controls how Envoy routes read commands to Redis nodes. This is currently
        /// supported for Redis Cluster. All ReadPolicy settings except MASTER may return stale data
        /// because replication is asynchronous and requires some delay. You need to ensure that your
        /// application can tolerate stale data.
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
        pub enum ReadPolicy {
            /// Default mode. Read from the current primary node.
            Master = 0,
            /// Read from the primary, but if it is unavailable, read from replica nodes.
            PreferMaster = 1,
            /// Read from replica nodes. If multiple replica nodes are present within a shard, a random
            /// node is selected. Healthy nodes have precedent over unhealthy nodes.
            Replica = 2,
            /// Read from the replica nodes (similar to REPLICA), but if all replicas are unavailable (not
            /// present or unhealthy), read from the primary.
            PreferReplica = 3,
            /// Read from any node of the cluster. A random node is selected among the primary and
            /// replicas, healthy nodes have precedent over unhealthy nodes.
            Any = 4,
        }
        impl ReadPolicy {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ReadPolicy::Master => "MASTER",
                    ReadPolicy::PreferMaster => "PREFER_MASTER",
                    ReadPolicy::Replica => "REPLICA",
                    ReadPolicy::PreferReplica => "PREFER_REPLICA",
                    ReadPolicy::Any => "ANY",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "MASTER" => Some(Self::Master),
                    "PREFER_MASTER" => Some(Self::PreferMaster),
                    "REPLICA" => Some(Self::Replica),
                    "PREFER_REPLICA" => Some(Self::PreferReplica),
                    "ANY" => Some(Self::Any),
                    _ => None,
                }
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PrefixRoutes {
        /// List of prefix routes.
        #[prost(message, repeated, tag = "1")]
        pub routes: ::prost::alloc::vec::Vec<prefix_routes::Route>,
        /// Indicates that prefix matching should be case insensitive.
        #[prost(bool, tag = "2")]
        pub case_insensitive: bool,
        /// Optional catch-all route to forward commands that doesn't match any of the routes. The
        /// catch-all route becomes required when no routes are specified.
        #[prost(message, optional, tag = "4")]
        pub catch_all_route: ::core::option::Option<prefix_routes::Route>,
    }
    /// Nested message and enum types in `PrefixRoutes`.
    pub mod prefix_routes {
        /// \[\#next-free-field: 7\]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Route {
            /// String prefix that must match the beginning of the keys. Envoy will always favor the
            /// longest match.
            #[prost(string, tag = "1")]
            pub prefix: ::prost::alloc::string::String,
            /// Indicates if the prefix needs to be removed from the key when forwarded.
            #[prost(bool, tag = "2")]
            pub remove_prefix: bool,
            /// Upstream cluster to forward the command to.
            #[prost(string, tag = "3")]
            pub cluster: ::prost::alloc::string::String,
            /// Indicates that the route has a request mirroring policy.
            #[prost(message, repeated, tag = "4")]
            pub request_mirror_policy: ::prost::alloc::vec::Vec<
                route::RequestMirrorPolicy,
            >,
            /// Indicates how redis key should be formatted. To substitute redis key into the formatting
            /// expression, use %KEY% as a string replacement command.
            #[prost(string, tag = "5")]
            pub key_formatter: ::prost::alloc::string::String,
            /// Indicates that the route has a read command policy
            #[prost(message, optional, tag = "6")]
            pub read_command_policy: ::core::option::Option<route::ReadCommandPolicy>,
        }
        /// Nested message and enum types in `Route`.
        pub mod route {
            /// The router is capable of shadowing traffic from one cluster to another. The current
            /// implementation is "fire and forget," meaning Envoy will not wait for the shadow cluster to
            /// respond before returning the response from the primary cluster. All normal statistics are
            /// collected for the shadow cluster making this feature useful for testing.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct RequestMirrorPolicy {
                /// Specifies the cluster that requests will be mirrored to. The cluster must
                /// exist in the cluster manager configuration.
                #[prost(string, tag = "1")]
                pub cluster: ::prost::alloc::string::String,
                /// If not specified or the runtime key is not present, all requests to the target cluster
                /// will be mirrored.
                ///
                /// If specified, Envoy will lookup the runtime key to get the percentage of requests to the
                /// mirror.
                #[prost(message, optional, tag = "2")]
                pub runtime_fraction: ::core::option::Option<
                    super::super::super::super::super::super::super::super::config::core::v3::RuntimeFractionalPercent,
                >,
                /// Set this to TRUE to only mirror write commands, this is effectively replicating the
                /// writes in a "fire and forget" manner.
                #[prost(bool, tag = "3")]
                pub exclude_read_commands: bool,
            }
            /// ReadCommandPolicy specifies that Envoy should route read commands to another cluster.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ReadCommandPolicy {
                #[prost(string, tag = "1")]
                pub cluster: ::prost::alloc::string::String,
            }
        }
    }
    /// RedisFault defines faults used for fault injection.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RedisFault {
        /// Fault type.
        #[prost(enumeration = "redis_fault::RedisFaultType", tag = "1")]
        pub fault_type: i32,
        /// Percentage of requests fault applies to.
        #[prost(message, optional, tag = "2")]
        pub fault_enabled: ::core::option::Option<
            super::super::super::super::super::super::config::core::v3::RuntimeFractionalPercent,
        >,
        /// Delay for all faults. If not set, defaults to zero
        #[prost(message, optional, tag = "3")]
        pub delay: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::Duration,
        >,
        /// Commands fault is restricted to, if any. If not set, fault applies to all commands
        /// other than auth and ping (due to special handling of those commands in Envoy).
        #[prost(string, repeated, tag = "4")]
        pub commands: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Nested message and enum types in `RedisFault`.
    pub mod redis_fault {
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
        pub enum RedisFaultType {
            /// Delays requests. This is the base fault; other faults can have delays added.
            Delay = 0,
            /// Returns errors on requests.
            Error = 1,
        }
        impl RedisFaultType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    RedisFaultType::Delay => "DELAY",
                    RedisFaultType::Error => "ERROR",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "DELAY" => Some(Self::Delay),
                    "ERROR" => Some(Self::Error),
                    _ => None,
                }
            }
        }
    }
    /// Configuration to limit reconnection rate to redis server to protect redis server
    /// from client reconnection storm.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConnectionRateLimit {
        /// Reconnection rate per sec. Rate limiting is implemented with TokenBucket.
        #[prost(uint32, tag = "1")]
        pub connection_rate_limit_per_sec: u32,
    }
}
/// RedisProtocolOptions specifies Redis upstream protocol options. This object is used in
/// :ref:`typed_extension_protocol_options<envoy_v3_api_field_config.cluster.v3.Cluster.typed_extension_protocol_options>`,
/// keyed by the name `envoy.filters.network.redis_proxy`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisProtocolOptions {
    /// Upstream server password as defined by the `requirepass` directive
    /// `<<https://redis.io/topics/config>`\_> in the server's configuration file.
    #[prost(message, optional, tag = "1")]
    pub auth_password: ::core::option::Option<
        super::super::super::super::super::config::core::v3::DataSource,
    >,
    /// Upstream server username as defined by the `user` directive
    /// `<<https://redis.io/topics/acl>`\_> in the server's configuration file.
    #[prost(message, optional, tag = "2")]
    pub auth_username: ::core::option::Option<
        super::super::super::super::super::config::core::v3::DataSource,
    >,
}
