/// \[\#next-free-field: 7\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessingMode {
    /// How to handle the request header. Default is "SEND".
    #[prost(enumeration = "processing_mode::HeaderSendMode", tag = "1")]
    pub request_header_mode: i32,
    /// How to handle the response header. Default is "SEND".
    #[prost(enumeration = "processing_mode::HeaderSendMode", tag = "2")]
    pub response_header_mode: i32,
    /// How to handle the request body. Default is "NONE".
    #[prost(enumeration = "processing_mode::BodySendMode", tag = "3")]
    pub request_body_mode: i32,
    /// How do handle the response body. Default is "NONE".
    #[prost(enumeration = "processing_mode::BodySendMode", tag = "4")]
    pub response_body_mode: i32,
    /// How to handle the request trailers. Default is "SKIP".
    #[prost(enumeration = "processing_mode::HeaderSendMode", tag = "5")]
    pub request_trailer_mode: i32,
    /// How to handle the response trailers. Default is "SKIP".
    #[prost(enumeration = "processing_mode::HeaderSendMode", tag = "6")]
    pub response_trailer_mode: i32,
}
/// Nested message and enum types in `ProcessingMode`.
pub mod processing_mode {
    /// Control how headers and trailers are handled
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
    pub enum HeaderSendMode {
        /// The default HeaderSendMode depends on which part of the message is being
        /// processed. By default, request and response headers are sent,
        /// while trailers are skipped.
        Default = 0,
        /// Send the header or trailer.
        Send = 1,
        /// Do not send the header or trailer.
        Skip = 2,
    }
    impl HeaderSendMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                HeaderSendMode::Default => "DEFAULT",
                HeaderSendMode::Send => "SEND",
                HeaderSendMode::Skip => "SKIP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEFAULT" => Some(Self::Default),
                "SEND" => Some(Self::Send),
                "SKIP" => Some(Self::Skip),
                _ => None,
            }
        }
    }
    /// Control how the request and response bodies are handled
    /// When body mutation by external processor is enabled, ext_proc filter will always remove
    /// the content length header in three cases below because content length can not be guaranteed
    /// to be set correctly:
    ///
    /// 1. STREAMED BodySendMode: header processing completes before body mutation comes back.
    /// 1. BUFFERED_PARTIAL BodySendMode: body is buffered and could be injected in different phases.
    /// 1. BUFFERED BodySendMode + SKIP HeaderSendMode: header processing (e.g., update content-length) is skipped.
    ///
    /// In Envoy's http1 codec implementation, removing content length will enable chunked transfer
    /// encoding whenever feasible. The recipient (either client or server) must be able
    /// to parse and decode the chunked transfer coding.
    /// (see `details in RFC9112 <<https://tools.ietf.org/html/rfc9112#section-7.1>`\_>).
    ///
    /// In BUFFERED BodySendMode + SEND HeaderSendMode, content length header is allowed but it is
    /// external processor's responsibility to set the content length correctly matched to the length
    /// of mutated body. If they don't match, the corresponding body mutation will be rejected and
    /// local reply will be sent with an error message.
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
    pub enum BodySendMode {
        /// Do not send the body at all. This is the default.
        None = 0,
        /// Stream the body to the server in pieces as they arrive at the
        /// proxy.
        Streamed = 1,
        /// Buffer the message body in memory and send the entire body at once.
        /// If the body exceeds the configured buffer limit, then the
        /// downstream system will receive an error.
        Buffered = 2,
        /// Buffer the message body in memory and send the entire body in one
        /// chunk. If the body exceeds the configured buffer limit, then the body contents
        /// up to the buffer limit will be sent.
        BufferedPartial = 3,
    }
    impl BodySendMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BodySendMode::None => "NONE",
                BodySendMode::Streamed => "STREAMED",
                BodySendMode::Buffered => "BUFFERED",
                BodySendMode::BufferedPartial => "BUFFERED_PARTIAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NONE" => Some(Self::None),
                "STREAMED" => Some(Self::Streamed),
                "BUFFERED" => Some(Self::Buffered),
                "BUFFERED_PARTIAL" => Some(Self::BufferedPartial),
                _ => None,
            }
        }
    }
}
/// The filter communicates with an external gRPC service called an "external processor"
/// that can do a variety of things with the request and response:
///
/// * Access and modify the HTTP headers on the request, response, or both
/// * Access and modify the HTTP request and response bodies
/// * Access and modify the dynamic stream metadata
/// * Immediately send an HTTP response downstream and terminate other processing
///
/// The filter communicates with the server using a gRPC bidirectional stream. After the initial
/// request, the external server is in control over what additional data is sent to it
/// and how it should be processed.
///
/// By implementing the protocol specified by the stream, the external server can choose:
///
/// * Whether it receives the response message at all
/// * Whether it receives the message body at all, in separate chunks, or as a single buffer
/// * Whether subsequent HTTP requests are transmitted synchronously or whether they are
///   sent asynchronously.
/// * To modify request or response trailers if they already exist
///
/// The filter supports up to six different processing steps. Each is represented by
/// a gRPC stream message that is sent to the external processor. For each message, the
/// processor must send a matching response.
///
/// * Request headers: Contains the headers from the original HTTP request.
/// * Request body: Delivered if they are present and sent in a single message if
///   the BUFFERED or BUFFERED_PARTIAL mode is chosen, in multiple messages if the
///   STREAMED mode is chosen, and not at all otherwise.
/// * Request trailers: Delivered if they are present and if the trailer mode is set
///   to SEND.
/// * Response headers: Contains the headers from the HTTP response. Keep in mind
///   that if the upstream system sends them before processing the request body that
///   this message may arrive before the complete body.
/// * Response body: Sent according to the processing mode like the request body.
/// * Response trailers: Delivered according to the processing mode like the
///   request trailers.
///
/// By default, the processor sends only the request and response headers messages.
/// This may be changed to include any of the six steps by changing the processing_mode
/// setting of the filter configuration, or by setting the mode_override of any response
/// from the external processor. The latter is only enabled if allow_mode_override is
/// set to true. This way, a processor may, for example, use information
/// in the request header to determine whether the message body must be examined, or whether
/// the proxy should simply stream it straight through.
///
/// All of this together allows a server to process the filter traffic in fairly
/// sophisticated ways. For example:
///
/// * A server may choose to examine all or part of the HTTP message bodies depending
///   on the content of the headers.
/// * A server may choose to immediately reject some messages based on their HTTP
///   headers (or other dynamic metadata) and more carefully examine others.
/// * A server may asynchronously monitor traffic coming through the filter by inspecting
///   headers, bodies, or both, and then decide to switch to a synchronous processing
///   mode, either permanently or temporarily.
///
/// The protocol itself is based on a bidirectional gRPC stream. Envoy will send the
/// server
/// :ref:`ProcessingRequest <envoy_v3_api_msg_service.ext_proc.v3.ProcessingRequest>`
/// messages, and the server must reply with
/// :ref:`ProcessingResponse <envoy_v3_api_msg_service.ext_proc.v3.ProcessingResponse>`.
///
/// Stats about each gRPC call are recorded in a :ref:`dynamic filter state <arch_overview_advanced_filter_state_sharing>` object in a namespace matching the filter
/// name.
///
/// \[\#next-free-field: 17\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalProcessor {
    /// Configuration for the gRPC service that the filter will communicate with.
    /// The filter supports both the "Envoy" and "Google" gRPC clients.
    #[prost(message, optional, tag = "1")]
    pub grpc_service: ::core::option::Option<
        super::super::super::super::super::config::core::v3::GrpcService,
    >,
    /// By default, if the gRPC stream cannot be established, or if it is closed
    /// prematurely with an error, the filter will fail. Specifically, if the
    /// response headers have not yet been delivered, then it will return a 500
    /// error downstream. If they have been delivered, then instead the HTTP stream to the
    /// downstream client will be reset.
    /// With this parameter set to true, however, then if the gRPC stream is prematurely closed
    /// or could not be opened, processing continues without error.
    #[prost(bool, tag = "2")]
    pub failure_mode_allow: bool,
    /// Specifies default options for how HTTP headers, trailers, and bodies are
    /// sent. See ProcessingMode for details.
    #[prost(message, optional, tag = "3")]
    pub processing_mode: ::core::option::Option<ProcessingMode>,
    /// \[\#not-implemented-hide:\]
    /// If true, send each part of the HTTP request or response specified by ProcessingMode
    /// asynchronously -- in other words, send the message on the gRPC stream and then continue
    /// filter processing. If false, which is the default, suspend filter execution after
    /// each message is sent to the remote service and wait up to "message_timeout"
    /// for a reply.
    #[prost(bool, tag = "4")]
    pub async_mode: bool,
    /// Envoy provides a number of :ref:`attributes <arch_overview_attributes>`
    /// for expressive policies. Each attribute name provided in this field will be
    /// matched against that list and populated in the request_headers message.
    /// See the :ref:`attribute documentation <arch_overview_request_attributes>`
    /// for the list of supported attributes and their types.
    #[prost(string, repeated, tag = "5")]
    pub request_attributes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Envoy provides a number of :ref:`attributes <arch_overview_attributes>`
    /// for expressive policies. Each attribute name provided in this field will be
    /// matched against that list and populated in the response_headers message.
    /// See the :ref:`attribute documentation <arch_overview_attributes>`
    /// for the list of supported attributes and their types.
    #[prost(string, repeated, tag = "6")]
    pub response_attributes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Specifies the timeout for each individual message sent on the stream and
    /// when the filter is running in synchronous mode. Whenever the proxy sends
    /// a message on the stream that requires a response, it will reset this timer,
    /// and will stop processing and return an error (subject to the processing mode)
    /// if the timer expires before a matching response is received. There is no
    /// timeout when the filter is running in asynchronous mode. Zero is a valid
    /// config which means the timer will be triggered immediately. If not
    /// configured, default is 200 milliseconds.
    #[prost(message, optional, tag = "7")]
    pub message_timeout: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// Optional additional prefix to use when emitting statistics. This allows to distinguish
    /// emitted statistics between configured *ext_proc* filters in an HTTP filter chain.
    #[prost(string, tag = "8")]
    pub stat_prefix: ::prost::alloc::string::String,
    /// Rules that determine what modifications an external processing server may
    /// make to message headers. If not set, all headers may be modified except
    /// for "host", ":authority", ":scheme", ":method", and headers that start
    /// with the header prefix set via
    /// :ref:`header_prefix <envoy_v3_api_field_config.bootstrap.v3.Bootstrap.header_prefix>`
    /// (which is usually "x-envoy").
    /// Note that changing headers such as "host" or ":authority" may not in itself
    /// change Envoy's routing decision, as routes can be cached. To also force the
    /// route to be recomputed, set the
    /// :ref:`clear_route_cache <envoy_v3_api_field_service.ext_proc.v3.CommonResponse.clear_route_cache>`
    /// field to true in the same response.
    #[prost(message, optional, tag = "9")]
    pub mutation_rules: ::core::option::Option<
        super::super::super::super::super::config::common::mutation_rules::v3::HeaderMutationRules,
    >,
    /// Specify the upper bound of
    /// :ref:`override_message_timeout <envoy_v3_api_field_service.ext_proc.v3.ProcessingResponse.override_message_timeout>`
    /// If not specified, by default it is 0, which will effectively disable the `override_message_timeout` API.
    #[prost(message, optional, tag = "10")]
    pub max_message_timeout: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// Prevents clearing the route-cache when the
    /// :ref:`clear_route_cache <envoy_v3_api_field_service.ext_proc.v3.CommonResponse.clear_route_cache>`
    /// field is set in an external processor response.
    #[prost(bool, tag = "11")]
    pub disable_clear_route_cache: bool,
    /// Allow headers matching the `forward_rules` to be forwarded to the external processing server.
    /// If not set, all headers are forwarded to the external processing server.
    #[prost(message, optional, tag = "12")]
    pub forward_rules: ::core::option::Option<HeaderForwardingRules>,
    /// Additional metadata to be added to the filter state for logging purposes. The metadata
    /// will be added to StreamInfo's filter state under the namespace corresponding to the
    /// ext_proc filter name.
    #[prost(message, optional, tag = "13")]
    pub filter_metadata: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Struct,
    >,
    /// If `allow_mode_override` is set to true, the filter config :ref:`processing_mode <envoy_v3_api_field_extensions.filters.http.ext_proc.v3.ExternalProcessor.processing_mode>`
    /// can be overridden by the response message from the external processing server
    /// :ref:`mode_override <envoy_v3_api_field_service.ext_proc.v3.ProcessingResponse.mode_override>`.
    /// If not set, `mode_override` API in the response message will be ignored.
    #[prost(bool, tag = "14")]
    pub allow_mode_override: bool,
    /// If set to true, ignore the
    /// :ref:`immediate_response <envoy_v3_api_field_service.ext_proc.v3.ProcessingResponse.immediate_response>`
    /// message in an external processor response. In such case, no local reply will be sent.
    /// Instead, the stream to the external processor will be closed. There will be no
    /// more external processing for this stream from now on.
    #[prost(bool, tag = "15")]
    pub disable_immediate_response: bool,
    /// Options related to the sending and receiving of dynamic metadata.
    #[prost(message, optional, tag = "16")]
    pub metadata_options: ::core::option::Option<MetadataOptions>,
}
/// The MetadataOptions structure defines options for the sending and receiving of
/// dynamic metadata. Specifically, which namespaces to send to the server, whether
/// metadata returned by the server may be written, and how that metadata may be written.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataOptions {
    /// Describes which typed or untyped dynamic metadata namespaces to forward to
    /// the external processing server.
    #[prost(message, optional, tag = "1")]
    pub forwarding_namespaces: ::core::option::Option<
        metadata_options::MetadataNamespaces,
    >,
    /// Describes which typed or untyped dynamic metadata namespaces to accept from
    /// the external processing server. Set to empty or leave unset to disallow writing
    /// any received dynamic metadata. Receiving of typed metadata is not supported.
    #[prost(message, optional, tag = "2")]
    pub receiving_namespaces: ::core::option::Option<
        metadata_options::MetadataNamespaces,
    >,
}
/// Nested message and enum types in `MetadataOptions`.
pub mod metadata_options {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MetadataNamespaces {
        /// Specifies a list of metadata namespaces whose values, if present,
        /// will be passed to the ext_proc service as an opaque *protobuf::Struct*.
        #[prost(string, repeated, tag = "1")]
        pub untyped: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Specifies a list of metadata namespaces whose values, if present,
        /// will be passed to the ext_proc service as a *protobuf::Any*. This allows
        /// envoy and the external processing server to share the protobuf message
        /// definition for safe parsing.
        #[prost(string, repeated, tag = "2")]
        pub typed: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// The HeaderForwardingRules structure specifies what headers are
/// allowed to be forwarded to the external processing server.
///
/// This works as below:
///
/// 1. If neither `allowed_headers` nor `disallowed_headers` is set, all headers are forwarded.
/// 1. If both `allowed_headers` and `disallowed_headers` are set, only headers in the
///    `allowed_headers` but not in the `disallowed_headers` are forwarded.
/// 1. If `allowed_headers` is set, and `disallowed_headers` is not set, only headers in
///    the `allowed_headers` are forwarded.
/// 1. If `disallowed_headers` is set, and `allowed_headers` is not set, all headers except
///    headers in the `disallowed_headers` are forwarded.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderForwardingRules {
    /// If set, specifically allow any header in this list to be forwarded to the external
    /// processing server. This can be overridden by the below `disallowed_headers`.
    #[prost(message, optional, tag = "1")]
    pub allowed_headers: ::core::option::Option<
        super::super::super::super::super::r#type::matcher::v3::ListStringMatcher,
    >,
    /// If set, specifically disallow any header in this list to be forwarded to the external
    /// processing server. This overrides the above `allowed_headers` if a header matches both.
    #[prost(message, optional, tag = "2")]
    pub disallowed_headers: ::core::option::Option<
        super::super::super::super::super::r#type::matcher::v3::ListStringMatcher,
    >,
}
/// Extra settings that may be added to per-route configuration for a
/// virtual host or cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtProcPerRoute {
    #[prost(oneof = "ext_proc_per_route::Override", tags = "1, 2")]
    pub r#override: ::core::option::Option<ext_proc_per_route::Override>,
}
/// Nested message and enum types in `ExtProcPerRoute`.
pub mod ext_proc_per_route {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Override {
        /// Disable the filter for this particular vhost or route.
        /// If disabled is specified in multiple per-filter-configs, the most specific one will be used.
        #[prost(bool, tag = "1")]
        Disabled(bool),
        /// Override aspects of the configuration for this route. A set of
        /// overrides in a more specific configuration will override a "disabled"
        /// flag set in a less-specific one.
        #[prost(message, tag = "2")]
        Overrides(super::ExtProcOverrides),
    }
}
/// Overrides that may be set on a per-route basis
/// \[\#next-free-field: 7\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtProcOverrides {
    /// Set a different processing mode for this route than the default.
    #[prost(message, optional, tag = "1")]
    pub processing_mode: ::core::option::Option<ProcessingMode>,
    /// \[\#not-implemented-hide:\]
    /// Set a different asynchronous processing option than the default.
    #[prost(bool, tag = "2")]
    pub async_mode: bool,
    /// \[\#not-implemented-hide:\]
    /// Set different optional attributes than the default setting of the
    /// `request_attributes` field.
    #[prost(string, repeated, tag = "3")]
    pub request_attributes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// \[\#not-implemented-hide:\]
    /// Set different optional properties than the default setting of the
    /// `response_attributes` field.
    #[prost(string, repeated, tag = "4")]
    pub response_attributes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Set a different gRPC service for this route than the default.
    #[prost(message, optional, tag = "5")]
    pub grpc_service: ::core::option::Option<
        super::super::super::super::super::config::core::v3::GrpcService,
    >,
    /// Options related to the sending and receiving of dynamic metadata.
    /// Lists of forwarding and receiving namespaces will be overridden in their entirety,
    /// meaning the most-specific config that specifies this override will be the final
    /// config used. It is the prerogative of the control plane to ensure this
    /// most-specific config contains the correct final overrides.
    #[prost(message, optional, tag = "6")]
    pub metadata_options: ::core::option::Option<MetadataOptions>,
}
