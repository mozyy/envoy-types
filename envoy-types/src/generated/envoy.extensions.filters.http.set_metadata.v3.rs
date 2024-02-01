#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    /// The metadata namespace.
    #[prost(string, tag = "1")]
    pub metadata_namespace: ::prost::alloc::string::String,
    /// Allow the filter to overwrite or merge with an existing value in the namespace.
    #[prost(bool, tag = "2")]
    pub allow_overwrite: bool,
    /// The value to place at the namespace. If `allow_overwrite`, this will
    /// overwrite or merge with any existing values in that namespace. See
    /// :ref:`the filter documentation <config_http_filters_set_metadata>` for
    /// more information on how this value is merged with potentially existing
    /// ones if `allow_overwrite` is configured. Only one of `value` and
    /// `typed_value` may be set.
    #[prost(message, optional, tag = "3")]
    pub value: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Struct,
    >,
    /// The value to place at the namespace. If `allow_overwrite`, this will
    /// overwrite any existing values in that namespace. Only one of `value` and
    /// `typed_value` may be set.
    #[prost(message, optional, tag = "4")]
    pub typed_value: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Any,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    /// The metadata namespace.
    /// This field is deprecated; please use `metadata` as replacement.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub metadata_namespace: ::prost::alloc::string::String,
    /// The untyped value to update the dynamic metadata namespace with. See
    /// :ref:`the filter documentation <config_http_filters_set_metadata>` for
    /// more information on how this value is merged with potentially existing
    /// ones.
    /// This field is deprecated; please use `metadata` as replacement.
    #[deprecated]
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Struct,
    >,
    /// Defines changes to be made to dynamic metadata.
    #[prost(message, repeated, tag = "3")]
    pub metadata: ::prost::alloc::vec::Vec<Metadata>,
}
