#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TlsInspector {
    /// Populate `JA3` fingerprint hash using data from the TLS Client Hello packet. Default is false.
    #[prost(message, optional, tag = "1")]
    pub enable_ja3_fingerprinting: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::BoolValue,
    >,
    /// The size in bytes of the initial buffer requested by the tls_inspector.
    /// If the filter needs to read additional bytes from the socket, the
    /// filter will double the buffer up to it's default maximum of 64KiB.
    /// If this size is not defined, defaults to maximum 64KiB that the
    /// tls inspector will consume.
    #[prost(message, optional, tag = "2")]
    pub initial_read_buffer_size: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
}
