#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoMessageLoggingConfig {
    #[prost(enumeration = "proto_message_logging_config::LogMode", tag = "3")]
    pub mode: i32,
    /// Specify the message logging info.
    /// The key is the fully qualified gRPC method name.
    /// `${package}.${Service}.${Method}`, like
    /// `endpoints.examples.bookstore.BookStore.GetShelf`
    ///
    /// The value is the message logging information for individual gRPC methods.
    #[prost(map = "string, message", tag = "4")]
    pub logging_by_method: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        MethodLogging,
    >,
    /// The proto descriptor set binary for the gRPC services.
    #[prost(oneof = "proto_message_logging_config::DescriptorSet", tags = "1, 2")]
    pub descriptor_set: ::core::option::Option<
        proto_message_logging_config::DescriptorSet,
    >,
}
/// Nested message and enum types in `ProtoMessageLoggingConfig`.
pub mod proto_message_logging_config {
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
    pub enum LogMode {
        Unspecified = 0,
        /// The filter will log the first and the last message for
        /// for streaming cases, containing
        /// client-side streaming, server-side streaming or bi-directional streaming.
        FirstAndLast = 1,
    }
    impl LogMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LogMode::Unspecified => "LogMode_UNSPECIFIED",
                LogMode::FirstAndLast => "FIRST_AND_LAST",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LogMode_UNSPECIFIED" => Some(Self::Unspecified),
                "FIRST_AND_LAST" => Some(Self::FirstAndLast),
                _ => None,
            }
        }
    }
    /// The proto descriptor set binary for the gRPC services.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DescriptorSet {
        /// It could be passed by a local file through `Datasource.filename` or
        /// embedded in the `Datasource.inline_bytes`.
        #[prost(message, tag = "1")]
        DataSource(
            super::super::super::super::super::super::config::core::v3::DataSource,
        ),
        /// Unimplemented, the key of proto descriptor TypedMetadata.
        /// Among filters depending on the proto descriptor, we can have a TypedMetadata
        /// for proto descriptors, so that these filters can share one copy of proto
        /// descriptor in memory.
        #[prost(string, tag = "2")]
        ProtoDescriptorTypedMetadata(::prost::alloc::string::String),
    }
}
/// This message can be used to support per route config approach later even
/// though the Istio doesn't support that so far.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MethodLogging {
    /// The mapping of field path to its LogDirective for request messages
    #[prost(map = "string, enumeration(method_logging::LogDirective)", tag = "2")]
    pub request_logging_by_field: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        i32,
    >,
    /// The mapping of field path to its LogDirective for response messages
    #[prost(map = "string, enumeration(method_logging::LogDirective)", tag = "3")]
    pub response_logging_by_field: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        i32,
    >,
}
/// Nested message and enum types in `MethodLogging`.
pub mod method_logging {
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
    pub enum LogDirective {
        Unspecified = 0,
        /// The value of this field will be logged.
        Log = 1,
        /// It should be only annotated on Message type fields so if the field isn't
        /// empty, an empty Struct will be logged.
        LogRedact = 2,
    }
    impl LogDirective {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LogDirective::Unspecified => "LogDirective_UNSPECIFIED",
                LogDirective::Log => "LOG",
                LogDirective::LogRedact => "LOG_REDACT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LogDirective_UNSPECIFIED" => Some(Self::Unspecified),
                "LOG" => Some(Self::Log),
                "LOG_REDACT" => Some(Self::LogRedact),
                _ => None,
            }
        }
    }
}
