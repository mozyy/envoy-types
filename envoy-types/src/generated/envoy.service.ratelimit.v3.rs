/// Main message for a rate limit request. The rate limit service is designed to be fully generic
/// in the sense that it can operate on arbitrary hierarchical key/value pairs. The loaded
/// configuration will parse the request and find the most specific limit to apply. In addition,
/// a RateLimitRequest can contain multiple "descriptors" to limit on. When multiple descriptors
/// are provided, the server will limit on *ALL* of them and return an OVER_LIMIT response if any
/// of them are over limit. This enables more complex application level rate limiting scenarios
/// if desired.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateLimitRequest {
    /// All rate limit requests must specify a domain. This enables the configuration to be per
    /// application without fear of overlap. E.g., "envoy".
    #[prost(string, tag = "1")]
    pub domain: ::prost::alloc::string::String,
    /// All rate limit requests must specify at least one RateLimitDescriptor. Each descriptor is
    /// processed by the service (see below). If any of the descriptors are over limit, the entire
    /// request is considered to be over limit.
    #[prost(message, repeated, tag = "2")]
    pub descriptors: ::prost::alloc::vec::Vec<
        super::super::super::extensions::common::ratelimit::v3::RateLimitDescriptor,
    >,
    /// Rate limit requests can optionally specify the number of hits a request adds to the matched
    /// limit. If the value is not set in the message, a request increases the matched limit by 1.
    #[prost(uint32, tag = "3")]
    pub hits_addend: u32,
}
/// A response from a ShouldRateLimit call.
/// \[\#next-free-field: 8\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateLimitResponse {
    /// The overall response code which takes into account all of the descriptors that were passed
    /// in the RateLimitRequest message.
    #[prost(enumeration = "rate_limit_response::Code", tag = "1")]
    pub overall_code: i32,
    /// A list of DescriptorStatus messages which matches the length of the descriptor list passed
    /// in the RateLimitRequest. This can be used by the caller to determine which individual
    /// descriptors failed and/or what the currently configured limits are for all of them.
    #[prost(message, repeated, tag = "2")]
    pub statuses: ::prost::alloc::vec::Vec<rate_limit_response::DescriptorStatus>,
    /// A list of headers to add to the response
    #[prost(message, repeated, tag = "3")]
    pub response_headers_to_add: ::prost::alloc::vec::Vec<
        super::super::super::config::core::v3::HeaderValue,
    >,
    /// A list of headers to add to the request when forwarded
    #[prost(message, repeated, tag = "4")]
    pub request_headers_to_add: ::prost::alloc::vec::Vec<
        super::super::super::config::core::v3::HeaderValue,
    >,
    /// A response body to send to the downstream client when the response code is not OK.
    #[prost(bytes = "vec", tag = "5")]
    pub raw_body: ::prost::alloc::vec::Vec<u8>,
    /// Optional response metadata that will be emitted as dynamic metadata to be consumed by the next
    /// filter. This metadata lives in a namespace specified by the canonical name of extension filter
    /// that requires it:
    ///
    /// * :ref:`envoy.filters.http.ratelimit <config_http_filters_ratelimit_dynamic_metadata>` for HTTP filter.
    /// * :ref:`envoy.filters.network.ratelimit <config_network_filters_ratelimit_dynamic_metadata>` for network filter.
    /// * :ref:`envoy.filters.thrift.rate_limit <config_thrift_filters_rate_limit_dynamic_metadata>` for Thrift filter.
    #[prost(message, optional, tag = "6")]
    pub dynamic_metadata: ::core::option::Option<
        super::super::super::super::google::protobuf::Struct,
    >,
    /// Quota is available for a request if its entire descriptor set has cached quota available.
    /// This is a union of all descriptors in the descriptor set. Clients can use the quota for future matches if and only if the descriptor set matches what was sent in the request that originated this response.
    ///
    /// If quota is available, a RLS request will not be made and the quota will be reduced by 1.
    /// If quota is not available (i.e., a cached entry doesn't exist for a RLS descriptor set), a RLS request will be triggered.
    /// If the server did not provide a quota, such as the quota message is empty then the request admission is determined by the
    /// :ref:`overall_code <envoy_v3_api_field_service.ratelimit.v3.RateLimitResponse.overall_code>`.
    ///
    /// If there is not sufficient quota and the cached entry exists for a RLS descriptor set is out-of-quota but not expired,
    /// the request will be treated as OVER_LIMIT.
    /// \[\#not-implemented-hide:\]
    #[prost(message, optional, tag = "7")]
    pub quota: ::core::option::Option<rate_limit_response::Quota>,
}
/// Nested message and enum types in `RateLimitResponse`.
pub mod rate_limit_response {
    /// Defines an actual rate limit in terms of requests per unit of time and the unit itself.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RateLimit {
        /// A name or description of this limit.
        #[prost(string, tag = "3")]
        pub name: ::prost::alloc::string::String,
        /// The number of requests per unit of time.
        #[prost(uint32, tag = "1")]
        pub requests_per_unit: u32,
        /// The unit of time.
        #[prost(enumeration = "rate_limit::Unit", tag = "2")]
        pub unit: i32,
    }
    /// Nested message and enum types in `RateLimit`.
    pub mod rate_limit {
        /// Identifies the unit of of time for rate limit.
        /// \[\#comment: replace by envoy/type/v3/ratelimit_unit.proto in v4\]
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
        pub enum Unit {
            /// The time unit is not known.
            Unknown = 0,
            /// The time unit representing a second.
            Second = 1,
            /// The time unit representing a minute.
            Minute = 2,
            /// The time unit representing an hour.
            Hour = 3,
            /// The time unit representing a day.
            Day = 4,
            /// The time unit representing a month.
            Month = 5,
            /// The time unit representing a year.
            Year = 6,
        }
        impl Unit {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Unit::Unknown => "UNKNOWN",
                    Unit::Second => "SECOND",
                    Unit::Minute => "MINUTE",
                    Unit::Hour => "HOUR",
                    Unit::Day => "DAY",
                    Unit::Month => "MONTH",
                    Unit::Year => "YEAR",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "UNKNOWN" => Some(Self::Unknown),
                    "SECOND" => Some(Self::Second),
                    "MINUTE" => Some(Self::Minute),
                    "HOUR" => Some(Self::Hour),
                    "DAY" => Some(Self::Day),
                    "MONTH" => Some(Self::Month),
                    "YEAR" => Some(Self::Year),
                    _ => None,
                }
            }
        }
    }
    /// Cacheable quota for responses.
    /// Quota can be granted at different levels: either for each individual descriptor or for the whole descriptor set.
    /// This is a certain number of requests over a period of time.
    /// The client may cache this result and apply the effective RateLimitResponse to future matching
    /// requests without querying rate limit service.
    ///
    /// When quota expires due to timeout, a new RLS request will also be made.
    /// The implementation may choose to preemptively query the rate limit server for more quota on or
    /// before expiration or before the available quota runs out.
    /// \[\#not-implemented-hide:\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Quota {
        /// Number of matching requests granted in quota. Must be 1 or more.
        #[prost(uint32, tag = "1")]
        pub requests: u32,
        /// The unique id that is associated with each Quota either at individual descriptor level or whole descriptor set level.
        ///
        /// For a matching policy with boolean logic, for example, match: "request.headers\['environment'\] == 'staging' || request.headers\['environment'\] == 'dev'"),
        /// the request_headers action produces a distinct list of descriptors for each possible value of the ‘environment’ header even though the granted quota is same.
        /// Thus, the client will use this id information (returned from RLS server) to correctly correlate the multiple descriptors/descriptor sets that have been granted with same quota (i.e., share the same quota among multiple descriptors or descriptor sets.)
        ///
        /// If id is empty, this id field will be ignored. If quota for the same id changes (e.g. due to configuration update), the old quota will be overridden by the new one. Shared quotas referenced by ID will still adhere to expiration after `valid_until`.
        #[prost(string, tag = "3")]
        pub id: ::prost::alloc::string::String,
        #[prost(oneof = "quota::ExpirationSpecifier", tags = "2")]
        pub expiration_specifier: ::core::option::Option<quota::ExpirationSpecifier>,
    }
    /// Nested message and enum types in `Quota`.
    pub mod quota {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ExpirationSpecifier {
            /// Point in time at which the quota expires.
            #[prost(message, tag = "2")]
            ValidUntil(
                super::super::super::super::super::super::google::protobuf::Timestamp,
            ),
        }
    }
    /// \[\#next-free-field: 6\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DescriptorStatus {
        /// The response code for an individual descriptor.
        #[prost(enumeration = "Code", tag = "1")]
        pub code: i32,
        /// The current limit as configured by the server. Useful for debugging, etc.
        #[prost(message, optional, tag = "2")]
        pub current_limit: ::core::option::Option<RateLimit>,
        /// The limit remaining in the current time unit.
        #[prost(uint32, tag = "3")]
        pub limit_remaining: u32,
        /// Duration until reset of the current limit window.
        #[prost(message, optional, tag = "4")]
        pub duration_until_reset: ::core::option::Option<
            super::super::super::super::super::google::protobuf::Duration,
        >,
        /// Quota is available for a request if its descriptor set has cached quota available for all
        /// descriptors.
        /// This is for each individual descriptor in the descriptor set. The client will perform matches for each individual descriptor against available per-descriptor quota.
        ///
        /// If quota is available, a RLS request will not be made and the quota will be reduced by 1 for
        /// all matching descriptors.
        ///
        /// If there is not sufficient quota, there are three cases:
        ///
        /// 1. A cached entry exists for a RLS descriptor that is out-of-quota, but not expired.
        ///    In this case, the request will be treated as OVER_LIMIT.
        /// 1. Some RLS descriptors have a cached entry that has valid quota but some RLS descriptors
        ///    have no cached entry. This will trigger a new RLS request.
        ///    When the result is returned, a single unit will be consumed from the quota for all
        ///    matching descriptors.
        ///    If the server did not provide a quota, such as the quota message is empty for some of
        ///    the descriptors, then the request admission is determined by the
        ///    :ref:`overall_code <envoy_v3_api_field_service.ratelimit.v3.RateLimitResponse.overall_code>`.
        /// 1. All RLS descriptors lack a cached entry, this will trigger a new RLS request,
        ///    When the result is returned, a single unit will be consumed from the quota for all
        ///    matching descriptors.
        ///    If the server did not provide a quota, such as the quota message is empty for some of
        ///    the descriptors, then the request admission is determined by the
        ///    :ref:`overall_code <envoy_v3_api_field_service.ratelimit.v3.RateLimitResponse.overall_code>`.
        ///    \[\#not-implemented-hide:\]
        #[prost(message, optional, tag = "5")]
        pub quota: ::core::option::Option<Quota>,
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
    pub enum Code {
        /// The response code is not known.
        Unknown = 0,
        /// The response code to notify that the number of requests are under limit.
        Ok = 1,
        /// The response code to notify that the number of requests are over limit.
        OverLimit = 2,
    }
    impl Code {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Code::Unknown => "UNKNOWN",
                Code::Ok => "OK",
                Code::OverLimit => "OVER_LIMIT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "OK" => Some(Self::Ok),
                "OVER_LIMIT" => Some(Self::OverLimit),
                _ => None,
            }
        }
    }
}
/// Generated client implementations.
pub mod rate_limit_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct RateLimitServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RateLimitServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> RateLimitServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            RateLimitServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Determine whether rate limiting should take place.
        pub async fn should_rate_limit(
            &mut self,
            request: impl tonic::IntoRequest<super::RateLimitRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RateLimitResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/envoy.service.ratelimit.v3.RateLimitService/ShouldRateLimit",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "envoy.service.ratelimit.v3.RateLimitService",
                        "ShouldRateLimit",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod rate_limit_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with RateLimitServiceServer.
    #[async_trait]
    pub trait RateLimitService: Send + Sync + 'static {
        /// Determine whether rate limiting should take place.
        async fn should_rate_limit(
            &self,
            request: tonic::Request<super::RateLimitRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RateLimitResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct RateLimitServiceServer<T: RateLimitService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: RateLimitService> RateLimitServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RateLimitServiceServer<T>
    where
        T: RateLimitService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/envoy.service.ratelimit.v3.RateLimitService/ShouldRateLimit" => {
                    #[allow(non_camel_case_types)]
                    struct ShouldRateLimitSvc<T: RateLimitService>(pub Arc<T>);
                    impl<
                        T: RateLimitService,
                    > tonic::server::UnaryService<super::RateLimitRequest>
                    for ShouldRateLimitSvc<T> {
                        type Response = super::RateLimitResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RateLimitRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RateLimitService>::should_rate_limit(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ShouldRateLimitSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: RateLimitService> Clone for RateLimitServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: RateLimitService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RateLimitService> tonic::server::NamedService for RateLimitServiceServer<T> {
        const NAME: &'static str = "envoy.service.ratelimit.v3.RateLimitService";
    }
}
