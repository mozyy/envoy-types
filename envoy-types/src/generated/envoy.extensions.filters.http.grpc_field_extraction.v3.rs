#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrpcFieldExtractionConfig {
    /// The proto descriptor set binary for the gRPC services.
    ///
    /// It could be passed by a local file through `Datasource.filename` or embedded in the
    /// `Datasource.inline_bytes`.
    #[prost(message, optional, tag = "1")]
    pub descriptor_set: ::core::option::Option<
        super::super::super::super::super::config::core::v3::DataSource,
    >,
    /// Specify the extraction info.
    /// The key is the fully qualified gRPC method name.
    /// `${package}.${Service}.${Method}`, like
    /// `endpoints.examples.bookstore.BookStore.GetShelf`
    ///
    /// The value is the field extractions for individual gRPC method.
    #[prost(map = "string, message", tag = "2")]
    pub extractions_by_method: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        FieldExtractions,
    >,
}
/// This message can be used to support per route config approach later even
/// though the Istio doesn't support that so far.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldExtractions {
    /// The field extractions for requests.
    /// The key is the field path within the grpc request.
    /// For example, we can define `foo.bar.name` if we want to extract
    /// `Request.foo.bar.name`.
    ///
    /// .. code-block:: proto
    ///
    /// message Request {
    /// Foo foo = 1;
    /// }
    ///
    /// message Foo {
    /// Bar bar = 1;
    /// }
    ///
    /// message Bar {
    /// string name = 1;
    /// }
    #[prost(map = "string, message", tag = "1")]
    pub request_field_extractions: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        RequestFieldValueDisposition,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestFieldValueDisposition {
    #[prost(oneof = "request_field_value_disposition::Disposition", tags = "1")]
    pub disposition: ::core::option::Option<
        request_field_value_disposition::Disposition,
    >,
}
/// Nested message and enum types in `RequestFieldValueDisposition`.
pub mod request_field_value_disposition {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Disposition {
        /// The dynamic metadata namespace. If empty, "envoy.filters.http.grpc_field_extraction" will be used by default.
        ///
        /// Unimplemented. Uses "envoy.filters.http.grpc_field_extraction" for now.
        #[prost(string, tag = "1")]
        DynamicMetadata(::prost::alloc::string::String),
    }
}
