#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonGeoipProviderConfig {
    /// Configuration for geolocation headers to add to request.
    #[prost(message, optional, tag = "1")]
    pub geo_headers_to_add: ::core::option::Option<
        common_geoip_provider_config::GeolocationHeadersToAdd,
    >,
}
/// Nested message and enum types in `CommonGeoipProviderConfig`.
pub mod common_geoip_provider_config {
    /// The set of geolocation headers to add to request. If any of the configured headers is present
    /// in the incoming request, it will be overridden by the :ref:`Geoip filter <config_http_filters_geoip>`.
    /// \[\#next-free-field: 10\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GeolocationHeadersToAdd {
        /// If set, the header will be used to populate the country ISO code associated with the IP address.
        #[prost(string, tag = "1")]
        pub country: ::prost::alloc::string::String,
        /// If set, the header will be used to populate the city associated with the IP address.
        #[prost(string, tag = "2")]
        pub city: ::prost::alloc::string::String,
        /// If set, the header will be used to populate the region ISO code associated with the IP address.
        /// The least specific subdivision will be selected as region value.
        #[prost(string, tag = "3")]
        pub region: ::prost::alloc::string::String,
        /// If set, the header will be used to populate the ASN associated with the IP address.
        #[prost(string, tag = "4")]
        pub asn: ::prost::alloc::string::String,
        /// If set, the IP address will be checked if it belongs to any type of anonymization network (e.g. VPN, public proxy etc)
        /// and header will be populated with the check result. Header value will be set to either "true" or "false" depending on the check result.
        #[prost(string, tag = "5")]
        pub is_anon: ::prost::alloc::string::String,
        /// If set, the IP address will be checked if it belongs to a VPN and header will be populated with the check result.
        /// Header value will be set to either "true" or "false" depending on the check result.
        #[prost(string, tag = "6")]
        pub anon_vpn: ::prost::alloc::string::String,
        /// If set, the IP address will be checked if it belongs to a hosting provider and header will be populated with the check result.
        /// Header value will be set to either "true" or "false" depending on the check result.
        #[prost(string, tag = "7")]
        pub anon_hosting: ::prost::alloc::string::String,
        /// If set, the IP address will be checked if it belongs to a TOR exit node and header will be populated with the check result.
        /// Header value will be set to either "true" or "false" depending on the check result.
        #[prost(string, tag = "8")]
        pub anon_tor: ::prost::alloc::string::String,
        /// If set, the IP address will be checked if it belongs to a public proxy and header will be populated with the check result.
        /// Header value will be set to either "true" or "false" depending on the check result.
        #[prost(string, tag = "9")]
        pub anon_proxy: ::prost::alloc::string::String,
    }
}
