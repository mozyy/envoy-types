/// Top level configuration for the tap filter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tap {
    /// Common configuration for the HTTP tap filter.
    #[prost(message, optional, tag = "1")]
    pub common_config: ::core::option::Option<
        super::super::super::super::common::tap::v3::CommonExtensionConfig,
    >,
    /// Indicates whether HTTP tap filter records the time stamp for request/response headers.
    /// Request headers time stamp is stored after receiving request headers.
    /// Response headers time stamp is stored after receiving response headers.
    #[prost(bool, tag = "2")]
    pub record_headers_received_time: bool,
    /// Indicates whether report downstream connection info
    #[prost(bool, tag = "3")]
    pub record_downstream_connection: bool,
}
