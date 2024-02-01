#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaxMindConfig {
    /// Full file path to the Maxmind city database, e.g. /etc/GeoLite2-City.mmdb.
    /// Database file is expected to have .mmdb extension.
    #[prost(string, tag = "1")]
    pub city_db_path: ::prost::alloc::string::String,
    /// Full file path to the Maxmind ASN database, e.g. /etc/GeoLite2-ASN.mmdb.
    /// Database file is expected to have .mmdb extension.
    #[prost(string, tag = "2")]
    pub isp_db_path: ::prost::alloc::string::String,
    /// Full file path to the Maxmind anonymous IP database, e.g. /etc/GeoIP2-Anonymous-IP.mmdb.
    /// Database file is expected to have .mmdb extension.
    #[prost(string, tag = "3")]
    pub anon_db_path: ::prost::alloc::string::String,
    /// Common provider configuration that specifies which geolocation headers will be populated with geolocation data.
    #[prost(message, optional, tag = "4")]
    pub common_provider_config: ::core::option::Option<
        super::super::common::v3::CommonGeoipProviderConfig,
    >,
}
