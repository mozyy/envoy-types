/// Wrapper for tapped body data. This includes HTTP request/response body, transport socket received
/// and transmitted data, etc.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Body {
    /// Specifies whether body data has been truncated to fit within the specified
    /// :ref:`max_buffered_rx_bytes <envoy_v3_api_field_config.tap.v3.OutputConfig.max_buffered_rx_bytes>` and
    /// :ref:`max_buffered_tx_bytes <envoy_v3_api_field_config.tap.v3.OutputConfig.max_buffered_tx_bytes>` settings.
    #[prost(bool, tag = "3")]
    pub truncated: bool,
    #[prost(oneof = "body::BodyType", tags = "1, 2")]
    pub body_type: ::core::option::Option<body::BodyType>,
}
/// Nested message and enum types in `Body`.
pub mod body {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum BodyType {
        /// Body data as bytes. By default, tap body data will be present in this field, as the proto
        /// `bytes` type can contain any valid byte.
        #[prost(bytes, tag = "1")]
        AsBytes(::prost::alloc::vec::Vec<u8>),
        /// Body data as string. This field is only used when the :ref:`JSON_BODY_AS_STRING <envoy_v3_api_enum_value_config.tap.v3.OutputSink.Format.JSON_BODY_AS_STRING>` sink
        /// format type is selected. See the documentation for that option for why this is useful.
        #[prost(string, tag = "2")]
        AsString(::prost::alloc::string::String),
    }
}
/// Connection properties.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connection {
    /// Local address.
    #[prost(message, optional, tag = "1")]
    pub local_address: ::core::option::Option<
        super::super::super::config::core::v3::Address,
    >,
    /// Remote address.
    #[prost(message, optional, tag = "2")]
    pub remote_address: ::core::option::Option<
        super::super::super::config::core::v3::Address,
    >,
}
/// A fully buffered HTTP trace message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpBufferedTrace {
    /// Request message.
    #[prost(message, optional, tag = "1")]
    pub request: ::core::option::Option<http_buffered_trace::Message>,
    /// Response message.
    #[prost(message, optional, tag = "2")]
    pub response: ::core::option::Option<http_buffered_trace::Message>,
    /// downstream connection
    #[prost(message, optional, tag = "3")]
    pub downstream_connection: ::core::option::Option<Connection>,
}
/// Nested message and enum types in `HttpBufferedTrace`.
pub mod http_buffered_trace {
    /// HTTP message wrapper.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Message {
        /// Message headers.
        #[prost(message, repeated, tag = "1")]
        pub headers: ::prost::alloc::vec::Vec<
            super::super::super::super::config::core::v3::HeaderValue,
        >,
        /// Message body.
        #[prost(message, optional, tag = "2")]
        pub body: ::core::option::Option<super::Body>,
        /// Message trailers.
        #[prost(message, repeated, tag = "3")]
        pub trailers: ::prost::alloc::vec::Vec<
            super::super::super::super::config::core::v3::HeaderValue,
        >,
        /// The timestamp after receiving the message headers.
        #[prost(message, optional, tag = "4")]
        pub headers_received_time: ::core::option::Option<
            super::super::super::super::super::google::protobuf::Timestamp,
        >,
    }
}
/// A streamed HTTP trace segment. Multiple segments make up a full trace.
/// \[\#next-free-field: 8\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpStreamedTraceSegment {
    /// Trace ID unique to the originating Envoy only. Trace IDs can repeat and should not be used
    /// for long term stable uniqueness.
    #[prost(uint64, tag = "1")]
    pub trace_id: u64,
    #[prost(
        oneof = "http_streamed_trace_segment::MessagePiece",
        tags = "2, 3, 4, 5, 6, 7"
    )]
    pub message_piece: ::core::option::Option<http_streamed_trace_segment::MessagePiece>,
}
/// Nested message and enum types in `HttpStreamedTraceSegment`.
pub mod http_streamed_trace_segment {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MessagePiece {
        /// Request headers.
        #[prost(message, tag = "2")]
        RequestHeaders(super::super::super::super::config::core::v3::HeaderMap),
        /// Request body chunk.
        #[prost(message, tag = "3")]
        RequestBodyChunk(super::Body),
        /// Request trailers.
        #[prost(message, tag = "4")]
        RequestTrailers(super::super::super::super::config::core::v3::HeaderMap),
        /// Response headers.
        #[prost(message, tag = "5")]
        ResponseHeaders(super::super::super::super::config::core::v3::HeaderMap),
        /// Response body chunk.
        #[prost(message, tag = "6")]
        ResponseBodyChunk(super::Body),
        /// Response trailers.
        #[prost(message, tag = "7")]
        ResponseTrailers(super::super::super::super::config::core::v3::HeaderMap),
    }
}
/// Event in a socket trace.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SocketEvent {
    /// Timestamp for event.
    #[prost(message, optional, tag = "1")]
    pub timestamp: ::core::option::Option<
        super::super::super::super::google::protobuf::Timestamp,
    >,
    /// Read or write with content as bytes string.
    #[prost(oneof = "socket_event::EventSelector", tags = "2, 3, 4")]
    pub event_selector: ::core::option::Option<socket_event::EventSelector>,
}
/// Nested message and enum types in `SocketEvent`.
pub mod socket_event {
    /// Data read by Envoy from the transport socket.
    ///
    /// TODO(htuch): Half-close for reads.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Read {
        /// Binary data read.
        #[prost(message, optional, tag = "1")]
        pub data: ::core::option::Option<super::Body>,
    }
    /// Data written by Envoy to the transport socket.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Write {
        /// Binary data written.
        #[prost(message, optional, tag = "1")]
        pub data: ::core::option::Option<super::Body>,
        /// Stream was half closed after this write.
        #[prost(bool, tag = "2")]
        pub end_stream: bool,
    }
    /// The connection was closed.
    ///
    /// TODO(mattklein123): Close event type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Closed {}
    /// Read or write with content as bytes string.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EventSelector {
        #[prost(message, tag = "2")]
        Read(Read),
        #[prost(message, tag = "3")]
        Write(Write),
        #[prost(message, tag = "4")]
        Closed(Closed),
    }
}
/// Sequence of read/write events that constitute a buffered trace on a socket.
/// \[\#next-free-field: 6\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SocketBufferedTrace {
    /// Trace ID unique to the originating Envoy only. Trace IDs can repeat and should not be used
    /// for long term stable uniqueness. Matches connection IDs used in Envoy logs.
    #[prost(uint64, tag = "1")]
    pub trace_id: u64,
    /// Connection properties.
    #[prost(message, optional, tag = "2")]
    pub connection: ::core::option::Option<Connection>,
    /// Sequence of observed events.
    #[prost(message, repeated, tag = "3")]
    pub events: ::prost::alloc::vec::Vec<SocketEvent>,
    /// Set to true if read events were truncated due to the :ref:`max_buffered_rx_bytes <envoy_v3_api_field_config.tap.v3.OutputConfig.max_buffered_rx_bytes>` setting.
    #[prost(bool, tag = "4")]
    pub read_truncated: bool,
    /// Set to true if write events were truncated due to the :ref:`max_buffered_tx_bytes <envoy_v3_api_field_config.tap.v3.OutputConfig.max_buffered_tx_bytes>` setting.
    #[prost(bool, tag = "5")]
    pub write_truncated: bool,
}
/// A streamed socket trace segment. Multiple segments make up a full trace.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SocketStreamedTraceSegment {
    /// Trace ID unique to the originating Envoy only. Trace IDs can repeat and should not be used
    /// for long term stable uniqueness. Matches connection IDs used in Envoy logs.
    #[prost(uint64, tag = "1")]
    pub trace_id: u64,
    #[prost(oneof = "socket_streamed_trace_segment::MessagePiece", tags = "2, 3")]
    pub message_piece: ::core::option::Option<
        socket_streamed_trace_segment::MessagePiece,
    >,
}
/// Nested message and enum types in `SocketStreamedTraceSegment`.
pub mod socket_streamed_trace_segment {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MessagePiece {
        /// Connection properties.
        #[prost(message, tag = "2")]
        Connection(super::Connection),
        /// Socket event.
        #[prost(message, tag = "3")]
        Event(super::SocketEvent),
    }
}
/// Wrapper for all fully buffered and streamed tap traces that Envoy emits. This is required for
/// sending traces over gRPC APIs or more easily persisting binary messages to files.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraceWrapper {
    #[prost(oneof = "trace_wrapper::Trace", tags = "1, 2, 3, 4")]
    pub trace: ::core::option::Option<trace_wrapper::Trace>,
}
/// Nested message and enum types in `TraceWrapper`.
pub mod trace_wrapper {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Trace {
        /// An HTTP buffered tap trace.
        #[prost(message, tag = "1")]
        HttpBufferedTrace(super::HttpBufferedTrace),
        /// An HTTP streamed tap trace segment.
        #[prost(message, tag = "2")]
        HttpStreamedTraceSegment(super::HttpStreamedTraceSegment),
        /// A socket buffered tap trace.
        #[prost(message, tag = "3")]
        SocketBufferedTrace(super::SocketBufferedTrace),
        /// A socket streamed tap trace segment.
        #[prost(message, tag = "4")]
        SocketStreamedTraceSegment(super::SocketStreamedTraceSegment),
    }
}
