/// \[\#not-implemented-hide:\]
/// Configuration for S2A transport socket. This allows Envoy clients to
/// configure how to offload mTLS handshakes to the S2A service.
/// <https://github.com/google/s2a-go#readme>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2aConfiguration {
    /// The address of the S2A. This can be an IP address or a hostname,
    /// followed by a port number.
    #[prost(string, tag = "1")]
    pub s2a_address: ::prost::alloc::string::String,
}
