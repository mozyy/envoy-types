/// \[\#next-free-field: 6\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TlsParameters {
    /// Minimum TLS protocol version. By default, it's `TLSv1_2` for both clients and servers.
    ///
    /// TLS protocol versions below TLSv1_2 require setting compatible ciphers with the
    /// `cipher_suites` setting as the default ciphers no longer include compatible ciphers.
    ///
    /// .. attention::
    ///
    /// Using TLS protocol versions below TLSv1_2 has serious security considerations and risks.
    #[prost(enumeration = "tls_parameters::TlsProtocol", tag = "1")]
    pub tls_minimum_protocol_version: i32,
    /// Maximum TLS protocol version. By default, it's `TLSv1_2` for clients and `TLSv1_3` for
    /// servers.
    #[prost(enumeration = "tls_parameters::TlsProtocol", tag = "2")]
    pub tls_maximum_protocol_version: i32,
    /// If specified, the TLS listener will only support the specified `cipher list <<https://commondatastorage.googleapis.com/chromium-boringssl-docs/ssl.h.html#Cipher-suite-configuration>`\_>
    /// when negotiating TLS 1.0-1.2 (this setting has no effect when negotiating TLS 1.3).
    ///
    /// If not specified, a default list will be used. Defaults are different for server (downstream) and
    /// client (upstream) TLS configurations.
    /// Defaults will change over time in response to security considerations; If you care, configure
    /// it instead of using the default.
    ///
    /// In non-FIPS builds, the default server cipher list is:
    ///
    /// .. code-block:: none
    ///
    /// \[ECDHE-ECDSA-AES128-GCM-SHA256|ECDHE-ECDSA-CHACHA20-POLY1305\]
    /// \[ECDHE-RSA-AES128-GCM-SHA256|ECDHE-RSA-CHACHA20-POLY1305\]
    /// ECDHE-ECDSA-AES256-GCM-SHA384
    /// ECDHE-RSA-AES256-GCM-SHA384
    ///
    /// In builds using :ref:`BoringSSL FIPS <arch_overview_ssl_fips>`, the default server cipher list is:
    ///
    /// .. code-block:: none
    ///
    /// ECDHE-ECDSA-AES128-GCM-SHA256
    /// ECDHE-RSA-AES128-GCM-SHA256
    /// ECDHE-ECDSA-AES256-GCM-SHA384
    /// ECDHE-RSA-AES256-GCM-SHA384
    ///
    /// In non-FIPS builds, the default client cipher list is:
    ///
    /// .. code-block:: none
    ///
    /// \[ECDHE-ECDSA-AES128-GCM-SHA256|ECDHE-ECDSA-CHACHA20-POLY1305\]
    /// \[ECDHE-RSA-AES128-GCM-SHA256|ECDHE-RSA-CHACHA20-POLY1305\]
    /// ECDHE-ECDSA-AES256-GCM-SHA384
    /// ECDHE-RSA-AES256-GCM-SHA384
    ///
    /// In builds using :ref:`BoringSSL FIPS <arch_overview_ssl_fips>`, the default client cipher list is:
    ///
    /// .. code-block:: none
    ///
    /// ECDHE-ECDSA-AES128-GCM-SHA256
    /// ECDHE-RSA-AES128-GCM-SHA256
    /// ECDHE-ECDSA-AES256-GCM-SHA384
    /// ECDHE-RSA-AES256-GCM-SHA384
    #[prost(string, repeated, tag = "3")]
    pub cipher_suites: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If specified, the TLS connection will only support the specified ECDH
    /// curves. If not specified, the default curves will be used.
    ///
    /// In non-FIPS builds, the default curves are:
    ///
    /// .. code-block:: none
    ///
    /// X25519
    /// P-256
    ///
    /// In builds using :ref:`BoringSSL FIPS <arch_overview_ssl_fips>`, the default curve is:
    ///
    /// .. code-block:: none
    ///
    /// P-256
    #[prost(string, repeated, tag = "4")]
    pub ecdh_curves: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If specified, the TLS connection will only support the specified signature algorithms.
    /// The list is ordered by preference.
    /// If not specified, the default signature algorithms defined by BoringSSL will be used.
    ///
    /// Default signature algorithms selected by BoringSSL (may be out of date):
    ///
    /// .. code-block:: none
    ///
    /// ecdsa_secp256r1_sha256
    /// rsa_pss_rsae_sha256
    /// rsa_pkcs1_sha256
    /// ecdsa_secp384r1_sha384
    /// rsa_pss_rsae_sha384
    /// rsa_pkcs1_sha384
    /// rsa_pss_rsae_sha512
    /// rsa_pkcs1_sha512
    /// rsa_pkcs1_sha1
    ///
    /// Signature algorithms supported by BoringSSL (may be out of date):
    ///
    /// .. code-block:: none
    ///
    /// rsa_pkcs1_sha256
    /// rsa_pkcs1_sha384
    /// rsa_pkcs1_sha512
    /// ecdsa_secp256r1_sha256
    /// ecdsa_secp384r1_sha384
    /// ecdsa_secp521r1_sha512
    /// rsa_pss_rsae_sha256
    /// rsa_pss_rsae_sha384
    /// rsa_pss_rsae_sha512
    /// ed25519
    /// rsa_pkcs1_sha1
    /// ecdsa_sha1
    #[prost(string, repeated, tag = "5")]
    pub signature_algorithms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `TlsParameters`.
pub mod tls_parameters {
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
    pub enum TlsProtocol {
        /// Envoy will choose the optimal TLS version.
        TlsAuto = 0,
        /// TLS 1.0
        TlSv10 = 1,
        /// TLS 1.1
        TlSv11 = 2,
        /// TLS 1.2
        TlSv12 = 3,
        /// TLS 1.3
        TlSv13 = 4,
    }
    impl TlsProtocol {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TlsProtocol::TlsAuto => "TLS_AUTO",
                TlsProtocol::TlSv10 => "TLSv1_0",
                TlsProtocol::TlSv11 => "TLSv1_1",
                TlsProtocol::TlSv12 => "TLSv1_2",
                TlsProtocol::TlSv13 => "TLSv1_3",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TLS_AUTO" => Some(Self::TlsAuto),
                "TLSv1_0" => Some(Self::TlSv10),
                "TLSv1_1" => Some(Self::TlSv11),
                "TLSv1_2" => Some(Self::TlSv12),
                "TLSv1_3" => Some(Self::TlSv13),
                _ => None,
            }
        }
    }
}
/// BoringSSL private key method configuration. The private key methods are used for external
/// (potentially asynchronous) signing and decryption operations. Some use cases for private key
/// methods would be TPM support and TLS acceleration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateKeyProvider {
    /// Private key method provider name. The name must match a
    /// supported private key method provider type.
    #[prost(string, tag = "1")]
    pub provider_name: ::prost::alloc::string::String,
    /// If the private key provider isn't available (eg. the required hardware capability doesn't existed),
    /// Envoy will fallback to the BoringSSL default implementation when the `fallback` is true.
    /// The default value is `false`.
    #[prost(bool, tag = "4")]
    pub fallback: bool,
    /// Private key method provider specific configuration.
    #[prost(oneof = "private_key_provider::ConfigType", tags = "3")]
    pub config_type: ::core::option::Option<private_key_provider::ConfigType>,
}
/// Nested message and enum types in `PrivateKeyProvider`.
pub mod private_key_provider {
    /// Private key method provider specific configuration.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConfigType {
        #[prost(message, tag = "3")]
        TypedConfig(super::super::super::super::super::super::google::protobuf::Any),
    }
}
/// \[\#next-free-field: 9\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TlsCertificate {
    /// The TLS certificate chain.
    ///
    /// If `certificate_chain` is a filesystem path, a watch will be added to the
    /// parent directory for any file moves to support rotation. This currently
    /// only applies to dynamic secrets, when the `TlsCertificate` is delivered via
    /// SDS.
    #[prost(message, optional, tag = "1")]
    pub certificate_chain: ::core::option::Option<
        super::super::super::super::config::core::v3::DataSource,
    >,
    /// The TLS private key.
    ///
    /// If `private_key` is a filesystem path, a watch will be added to the parent
    /// directory for any file moves to support rotation. This currently only
    /// applies to dynamic secrets, when the `TlsCertificate` is delivered via SDS.
    #[prost(message, optional, tag = "2")]
    pub private_key: ::core::option::Option<
        super::super::super::super::config::core::v3::DataSource,
    >,
    /// `Pkcs12` data containing TLS certificate, chain, and private key.
    ///
    /// If `pkcs12` is a filesystem path, the file will be read, but no watch will
    /// be added to the parent directory, since `pkcs12` isn't used by SDS.
    /// This field is mutually exclusive with `certificate_chain`, `private_key` and `private_key_provider`.
    /// This can't be marked as `oneof` due to API compatibility reasons. Setting
    /// both :ref:`private_key <envoy_v3_api_field_extensions.transport_sockets.tls.v3.TlsCertificate.private_key>`,
    /// :ref:`certificate_chain <envoy_v3_api_field_extensions.transport_sockets.tls.v3.TlsCertificate.certificate_chain>`,
    /// or :ref:`private_key_provider <envoy_v3_api_field_extensions.transport_sockets.tls.v3.TlsCertificate.private_key_provider>`
    /// and :ref:`pkcs12 <envoy_v3_api_field_extensions.transport_sockets.tls.v3.TlsCertificate.pkcs12>`
    /// fields will result in an error. Use :ref:`password <envoy_v3_api_field_extensions.transport_sockets.tls.v3.TlsCertificate.password>`
    /// to specify the password to unprotect the `PKCS12` data, if necessary.
    #[prost(message, optional, tag = "8")]
    pub pkcs12: ::core::option::Option<
        super::super::super::super::config::core::v3::DataSource,
    >,
    /// If specified, updates of file-based `certificate_chain` and `private_key`
    /// sources will be triggered by this watch. The certificate/key pair will be
    /// read together and validated for atomic read consistency (i.e. no
    /// intervening modification occurred between cert/key read, verified by file
    /// hash comparisons). This allows explicit control over the path watched, by
    /// default the parent directories of the filesystem paths in
    /// `certificate_chain` and `private_key` are watched if this field is not
    /// specified. This only applies when a `TlsCertificate` is delivered by SDS
    /// with references to filesystem paths. See the :ref:`SDS key rotation <sds_key_rotation>` documentation for further details.
    #[prost(message, optional, tag = "7")]
    pub watched_directory: ::core::option::Option<
        super::super::super::super::config::core::v3::WatchedDirectory,
    >,
    /// BoringSSL private key method provider. This is an alternative to :ref:`private_key <envoy_v3_api_field_extensions.transport_sockets.tls.v3.TlsCertificate.private_key>` field. This can't be
    /// marked as `oneof` due to API compatibility reasons. Setting both :ref:`private_key <envoy_v3_api_field_extensions.transport_sockets.tls.v3.TlsCertificate.private_key>` and
    /// :ref:`private_key_provider <envoy_v3_api_field_extensions.transport_sockets.tls.v3.TlsCertificate.private_key_provider>` fields will result in an
    /// error.
    #[prost(message, optional, tag = "6")]
    pub private_key_provider: ::core::option::Option<PrivateKeyProvider>,
    /// The password to decrypt the TLS private key. If this field is not set, it is assumed that the
    /// TLS private key is not password encrypted.
    #[prost(message, optional, tag = "3")]
    pub password: ::core::option::Option<
        super::super::super::super::config::core::v3::DataSource,
    >,
    /// The OCSP response to be stapled with this certificate during the handshake.
    /// The response must be DER-encoded and may only be  provided via `filename` or
    /// `inline_bytes`. The response may pertain to only one certificate.
    #[prost(message, optional, tag = "4")]
    pub ocsp_staple: ::core::option::Option<
        super::super::super::super::config::core::v3::DataSource,
    >,
    /// \[\#not-implemented-hide:\]
    #[prost(message, repeated, tag = "5")]
    pub signed_certificate_timestamp: ::prost::alloc::vec::Vec<
        super::super::super::super::config::core::v3::DataSource,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TlsSessionTicketKeys {
    /// Keys for encrypting and decrypting TLS session tickets. The
    /// first key in the array contains the key to encrypt all new sessions created by this context.
    /// All keys are candidates for decrypting received tickets. This allows for easy rotation of keys
    /// by, for example, putting the new key first, and the previous key second.
    ///
    /// If :ref:`session_ticket_keys <envoy_v3_api_field_extensions.transport_sockets.tls.v3.DownstreamTlsContext.session_ticket_keys>`
    /// is not specified, the TLS library will still support resuming sessions via tickets, but it will
    /// use an internally-generated and managed key, so sessions cannot be resumed across hot restarts
    /// or on different hosts.
    ///
    /// Each key must contain exactly 80 bytes of cryptographically-secure random data. For
    /// example, the output of `openssl rand 80`.
    ///
    /// .. attention::
    ///
    /// Using this feature has serious security considerations and risks. Improper handling of keys
    /// may result in loss of secrecy in connections, even if ciphers supporting perfect forward
    /// secrecy are used. See <https://www.imperialviolet.org/2013/06/27/botchingpfs.html> for some
    /// discussion. To minimize the risk, you must:
    ///
    /// * Keep the session ticket keys at least as secure as your TLS certificate private keys
    /// * Rotate session ticket keys at least daily, and preferably hourly
    /// * Always generate keys using a cryptographically-secure random data source
    #[prost(message, repeated, tag = "1")]
    pub keys: ::prost::alloc::vec::Vec<
        super::super::super::super::config::core::v3::DataSource,
    >,
}
/// Indicates a certificate to be obtained from a named CertificateProvider plugin instance.
/// The plugin instances are defined in the client's bootstrap file.
/// The plugin allows certificates to be fetched/refreshed over the network asynchronously with
/// respect to the TLS handshake.
/// \[\#not-implemented-hide:\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateProviderPluginInstance {
    /// Provider instance name. If not present, defaults to "default".
    ///
    /// Instance names should generally be defined not in terms of the underlying provider
    /// implementation (e.g., "file_watcher") but rather in terms of the function of the
    /// certificates (e.g., "foo_deployment_identity").
    #[prost(string, tag = "1")]
    pub instance_name: ::prost::alloc::string::String,
    /// Opaque name used to specify certificate instances or types. For example, "ROOTCA" to specify
    /// a root-certificate (validation context) or "example.com" to specify a certificate for a
    /// particular domain. Not all provider instances will actually use this field, so the value
    /// defaults to the empty string.
    #[prost(string, tag = "2")]
    pub certificate_name: ::prost::alloc::string::String,
}
/// Matcher for subject alternative names, to match both type and value of the SAN.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubjectAltNameMatcher {
    /// Specification of type of SAN. Note that the default enum value is an invalid choice.
    #[prost(enumeration = "subject_alt_name_matcher::SanType", tag = "1")]
    pub san_type: i32,
    /// Matcher for SAN value.
    #[prost(message, optional, tag = "2")]
    pub matcher: ::core::option::Option<
        super::super::super::super::r#type::matcher::v3::StringMatcher,
    >,
}
/// Nested message and enum types in `SubjectAltNameMatcher`.
pub mod subject_alt_name_matcher {
    /// Indicates the choice of GeneralName as defined in section 4.2.1.5 of RFC 5280 to match
    /// against.
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
    pub enum SanType {
        Unspecified = 0,
        Email = 1,
        Dns = 2,
        Uri = 3,
        IpAddress = 4,
    }
    impl SanType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SanType::Unspecified => "SAN_TYPE_UNSPECIFIED",
                SanType::Email => "EMAIL",
                SanType::Dns => "DNS",
                SanType::Uri => "URI",
                SanType::IpAddress => "IP_ADDRESS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SAN_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "EMAIL" => Some(Self::Email),
                "DNS" => Some(Self::Dns),
                "URI" => Some(Self::Uri),
                "IP_ADDRESS" => Some(Self::IpAddress),
                _ => None,
            }
        }
    }
}
/// \[\#next-free-field: 17\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateValidationContext {
    /// TLS certificate data containing certificate authority certificates to use in verifying
    /// a presented peer certificate (e.g. server certificate for clusters or client certificate
    /// for listeners). If not specified and a peer certificate is presented it will not be
    /// verified. By default, a client certificate is optional, unless one of the additional
    /// options (:ref:`require_client_certificate <envoy_v3_api_field_extensions.transport_sockets.tls.v3.DownstreamTlsContext.require_client_certificate>`,
    /// :ref:`verify_certificate_spki <envoy_v3_api_field_extensions.transport_sockets.tls.v3.CertificateValidationContext.verify_certificate_spki>`,
    /// :ref:`verify_certificate_hash <envoy_v3_api_field_extensions.transport_sockets.tls.v3.CertificateValidationContext.verify_certificate_hash>`, or
    /// :ref:`match_typed_subject_alt_names <envoy_v3_api_field_extensions.transport_sockets.tls.v3.CertificateValidationContext.match_typed_subject_alt_names>`) is also
    /// specified.
    ///
    /// It can optionally contain certificate revocation lists, in which case Envoy will verify
    /// that the presented peer certificate has not been revoked by one of the included CRLs. Note
    /// that if a CRL is provided for any certificate authority in a trust chain, a CRL must be
    /// provided for all certificate authorities in that chain. Failure to do so will result in
    /// verification failure for both revoked and unrevoked certificates from that chain.
    /// The behavior of requiring all certificates to contain CRLs can be altered by
    /// setting :ref:`only_verify_leaf_cert_crl <envoy_v3_api_field_extensions.transport_sockets.tls.v3.CertificateValidationContext.only_verify_leaf_cert_crl>`
    /// true. If set to true, only the final certificate in the chain undergoes CRL verification.
    ///
    /// See :ref:`the TLS overview <arch_overview_ssl_enabling_verification>` for a list of common
    /// system CA locations.
    ///
    /// If `trusted_ca` is a filesystem path, a watch will be added to the parent
    /// directory for any file moves to support rotation. This currently only
    /// applies to dynamic secrets, when the `CertificateValidationContext` is
    /// delivered via SDS.
    ///
    /// X509_V_FLAG_PARTIAL_CHAIN is set by default, so non-root/intermediate ca certificate in `trusted_ca`
    /// can be treated as trust anchor as well. It allows verification with building valid partial chain instead
    /// of a full chain.
    ///
    /// Only one of `trusted_ca` and `ca_certificate_provider_instance` may be specified.
    ///
    /// \[\#next-major-version: This field and watched_directory below should ideally be moved into a
    /// separate sub-message, since there's no point in specifying the latter field without this one.\]
    #[prost(message, optional, tag = "1")]
    pub trusted_ca: ::core::option::Option<
        super::super::super::super::config::core::v3::DataSource,
    >,
    /// Certificate provider instance for fetching TLS certificates.
    ///
    /// Only one of `trusted_ca` and `ca_certificate_provider_instance` may be specified.
    /// \[\#not-implemented-hide:\]
    #[prost(message, optional, tag = "13")]
    pub ca_certificate_provider_instance: ::core::option::Option<
        CertificateProviderPluginInstance,
    >,
    /// If specified, updates of a file-based `trusted_ca` source will be triggered
    /// by this watch. This allows explicit control over the path watched, by
    /// default the parent directory of the filesystem path in `trusted_ca` is
    /// watched if this field is not specified. This only applies when a
    /// `CertificateValidationContext` is delivered by SDS with references to
    /// filesystem paths. See the :ref:`SDS key rotation <sds_key_rotation>`
    /// documentation for further details.
    #[prost(message, optional, tag = "11")]
    pub watched_directory: ::core::option::Option<
        super::super::super::super::config::core::v3::WatchedDirectory,
    >,
    /// An optional list of base64-encoded SHA-256 hashes. If specified, Envoy will verify that the
    /// SHA-256 of the DER-encoded Subject Public Key Information (SPKI) of the presented certificate
    /// matches one of the specified values.
    ///
    /// A base64-encoded SHA-256 of the Subject Public Key Information (SPKI) of the certificate
    /// can be generated with the following command:
    ///
    /// .. code-block:: bash
    ///
    /// $ openssl x509 -in path/to/client.crt -noout -pubkey
    /// \| openssl pkey -pubin -outform DER
    /// \| openssl dgst -sha256 -binary
    /// \| openssl enc -base64
    /// NvqYIYSbgK2vCJpQhObf77vv+bQWtc5ek5RIOwPiC9A=
    ///
    /// This is the format used in HTTP Public Key Pinning.
    ///
    /// When both:
    /// :ref:`verify_certificate_hash <envoy_v3_api_field_extensions.transport_sockets.tls.v3.CertificateValidationContext.verify_certificate_hash>` and
    /// :ref:`verify_certificate_spki <envoy_v3_api_field_extensions.transport_sockets.tls.v3.CertificateValidationContext.verify_certificate_spki>` are specified,
    /// a hash matching value from either of the lists will result in the certificate being accepted.
    ///
    /// .. attention::
    ///
    /// This option is preferred over :ref:`verify_certificate_hash <envoy_v3_api_field_extensions.transport_sockets.tls.v3.CertificateValidationContext.verify_certificate_hash>`,
    /// because SPKI is tied to a private key, so it doesn't change when the certificate
    /// is renewed using the same private key.
    #[prost(string, repeated, tag = "3")]
    pub verify_certificate_spki: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// An optional list of hex-encoded SHA-256 hashes. If specified, Envoy will verify that
    /// the SHA-256 of the DER-encoded presented certificate matches one of the specified values.
    ///
    /// A hex-encoded SHA-256 of the certificate can be generated with the following command:
    ///
    /// .. code-block:: bash
    ///
    /// $ openssl x509 -in path/to/client.crt -outform DER | openssl dgst -sha256 | cut -d" " -f2
    /// df6ff72fe9116521268f6f2dd4966f51df479883fe7037b39f75916ac3049d1a
    ///
    /// A long hex-encoded and colon-separated SHA-256 (a.k.a. "fingerprint") of the certificate
    /// can be generated with the following command:
    ///
    /// .. code-block:: bash
    ///
    /// $ openssl x509 -in path/to/client.crt -noout -fingerprint -sha256 | cut -d"=" -f2
    /// DF:6F:F7:2F:E9:11:65:21:26:8F:6F:2D:D4:96:6F:51:DF:47:98:83:FE:70:37:B3:9F:75:91:6A:C3:04:9D:1A
    ///
    /// Both of those formats are acceptable.
    ///
    /// When both:
    /// :ref:`verify_certificate_hash <envoy_v3_api_field_extensions.transport_sockets.tls.v3.CertificateValidationContext.verify_certificate_hash>` and
    /// :ref:`verify_certificate_spki <envoy_v3_api_field_extensions.transport_sockets.tls.v3.CertificateValidationContext.verify_certificate_spki>` are specified,
    /// a hash matching value from either of the lists will result in the certificate being accepted.
    #[prost(string, repeated, tag = "2")]
    pub verify_certificate_hash: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// An optional list of Subject Alternative name matchers. If specified, Envoy will verify that the
    /// Subject Alternative Name of the presented certificate matches one of the specified matchers.
    /// The matching uses "any" semantics, that is to say, the SAN is verified if at least one matcher is
    /// matched.
    ///
    /// When a certificate has wildcard DNS SAN entries, to match a specific client, it should be
    /// configured with exact match type in the :ref:`string matcher <envoy_v3_api_msg_type.matcher.v3.StringMatcher>`.
    /// For example if the certificate has "\*.example.com" as DNS SAN entry, to allow only "api.example.com",
    /// it should be configured as shown below.
    ///
    /// .. code-block:: yaml
    ///
    /// match_typed_subject_alt_names:
    ///
    /// * san_type: DNS
    ///   matcher:
    ///   exact: "api.example.com"
    ///
    /// .. attention::
    ///
    /// Subject Alternative Names are easily spoofable and verifying only them is insecure,
    /// therefore this option must be used together with :ref:`trusted_ca <envoy_v3_api_field_extensions.transport_sockets.tls.v3.CertificateValidationContext.trusted_ca>`.
    #[prost(message, repeated, tag = "15")]
    pub match_typed_subject_alt_names: ::prost::alloc::vec::Vec<SubjectAltNameMatcher>,
    /// This field is deprecated in favor of
    /// :ref:`match_typed_subject_alt_names <envoy_v3_api_field_extensions.transport_sockets.tls.v3.CertificateValidationContext.match_typed_subject_alt_names>`.
    /// Note that if both this field and :ref:`match_typed_subject_alt_names <envoy_v3_api_field_extensions.transport_sockets.tls.v3.CertificateValidationContext.match_typed_subject_alt_names>`
    /// are specified, the former (deprecated field) is ignored.
    #[deprecated]
    #[prost(message, repeated, tag = "9")]
    pub match_subject_alt_names: ::prost::alloc::vec::Vec<
        super::super::super::super::r#type::matcher::v3::StringMatcher,
    >,
    /// \[\#not-implemented-hide:\] Must present signed certificate time-stamp.
    #[prost(message, optional, tag = "6")]
    pub require_signed_certificate_timestamp: ::core::option::Option<
        super::super::super::super::super::google::protobuf::BoolValue,
    >,
    /// An optional `certificate revocation list <<https://en.wikipedia.org/wiki/Certificate_revocation_list>`\_>
    /// (in PEM format). If specified, Envoy will verify that the presented peer
    /// certificate has not been revoked by this CRL. If this DataSource contains
    /// multiple CRLs, all of them will be used. Note that if a CRL is provided
    /// for any certificate authority in a trust chain, a CRL must be provided
    /// for all certificate authorities in that chain. Failure to do so will
    /// result in verification failure for both revoked and unrevoked certificates
    /// from that chain. This default behavior can be altered by setting
    /// :ref:`only_verify_leaf_cert_crl <envoy_v3_api_field_extensions.transport_sockets.tls.v3.CertificateValidationContext.only_verify_leaf_cert_crl>` to
    /// true.
    ///
    /// If `crl` is a filesystem path, a watch will be added to the parent
    /// directory for any file moves to support rotation. This currently only
    /// applies to dynamic secrets, when the `CertificateValidationContext` is
    /// delivered via SDS.
    #[prost(message, optional, tag = "7")]
    pub crl: ::core::option::Option<
        super::super::super::super::config::core::v3::DataSource,
    >,
    /// If specified, Envoy will not reject expired certificates.
    #[prost(bool, tag = "8")]
    pub allow_expired_certificate: bool,
    /// Certificate trust chain verification mode.
    #[prost(
        enumeration = "certificate_validation_context::TrustChainVerification",
        tag = "10"
    )]
    pub trust_chain_verification: i32,
    /// The configuration of an extension specific certificate validator.
    /// If specified, all validation is done by the specified validator,
    /// and the behavior of all other validation settings is defined by the specified validator (and may be entirely ignored, unused, and unvalidated).
    /// Refer to the documentation for the specified validator. If you do not want a custom validation algorithm, do not set this field.
    /// \[\#extension-category: envoy.tls.cert_validator\]
    #[prost(message, optional, tag = "12")]
    pub custom_validator_config: ::core::option::Option<
        super::super::super::super::config::core::v3::TypedExtensionConfig,
    >,
    /// If this option is set to true, only the certificate at the end of the
    /// certificate chain will be subject to validation by :ref:`CRL <envoy_v3_api_field_extensions.transport_sockets.tls.v3.CertificateValidationContext.crl>`.
    #[prost(bool, tag = "14")]
    pub only_verify_leaf_cert_crl: bool,
    /// Defines maximum depth of a certificate chain accepted in verification, the default limit is 100, though this can be system-dependent.
    /// This number does not include the leaf but includes the trust anchor, so a depth of 1 allows the leaf and one CA certificate. If a trusted issuer
    /// appears in the chain, but in a depth larger than configured, the certificate validation will fail.
    /// This matches the semantics of `SSL_CTX_set_verify_depth` in OpenSSL 1.0.x and older versions of BoringSSL. It differs from `SSL_CTX_set_verify_depth`
    /// in OpenSSL 1.1.x and newer versions of BoringSSL in that the trust anchor is included.
    /// Trusted issues are specified by setting :ref:`trusted_ca <envoy_v3_api_field_extensions.transport_sockets.tls.v3.CertificateValidationContext.trusted_ca>`
    #[prost(message, optional, tag = "16")]
    pub max_verify_depth: ::core::option::Option<
        super::super::super::super::super::google::protobuf::UInt32Value,
    >,
}
/// Nested message and enum types in `CertificateValidationContext`.
pub mod certificate_validation_context {
    /// Peer certificate verification mode.
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
    pub enum TrustChainVerification {
        /// Perform default certificate verification (e.g., against CA / verification lists)
        VerifyTrustChain = 0,
        /// Connections where the certificate fails verification will be permitted.
        /// For HTTP connections, the result of certificate verification can be used in route matching. (
        /// see :ref:`validated <envoy_v3_api_field_config.route.v3.RouteMatch.TlsContextMatchOptions.validated>` ).
        AcceptUntrusted = 1,
    }
    impl TrustChainVerification {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TrustChainVerification::VerifyTrustChain => "VERIFY_TRUST_CHAIN",
                TrustChainVerification::AcceptUntrusted => "ACCEPT_UNTRUSTED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VERIFY_TRUST_CHAIN" => Some(Self::VerifyTrustChain),
                "ACCEPT_UNTRUSTED" => Some(Self::AcceptUntrusted),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericSecret {
    /// Secret of generic type and is available to filters.
    #[prost(message, optional, tag = "1")]
    pub secret: ::core::option::Option<
        super::super::super::super::config::core::v3::DataSource,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SdsSecretConfig {
    /// Name by which the secret can be uniquely referred to. When both name and config are specified,
    /// then secret can be fetched and/or reloaded via SDS. When only name is specified, then secret
    /// will be loaded from static resources.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub sds_config: ::core::option::Option<
        super::super::super::super::config::core::v3::ConfigSource,
    >,
}
/// \[\#next-free-field: 6\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Secret {
    /// Name (FQDN, UUID, SPKI, SHA256, etc.) by which the secret can be uniquely referred to.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof = "secret::Type", tags = "2, 3, 4, 5")]
    pub r#type: ::core::option::Option<secret::Type>,
}
/// Nested message and enum types in `Secret`.
pub mod secret {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag = "2")]
        TlsCertificate(super::TlsCertificate),
        #[prost(message, tag = "3")]
        SessionTicketKeys(super::TlsSessionTicketKeys),
        #[prost(message, tag = "4")]
        ValidationContext(super::CertificateValidationContext),
        #[prost(message, tag = "5")]
        GenericSecret(super::GenericSecret),
    }
}
/// \[\#next-free-field: 6\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpstreamTlsContext {
    /// Common TLS context settings.
    ///
    /// .. attention::
    ///
    /// Server certificate verification is not enabled by default. Configure
    /// :ref:`trusted_ca<envoy_v3_api_field_extensions.transport_sockets.tls.v3.CertificateValidationContext.trusted_ca>` to enable
    /// verification.
    #[prost(message, optional, tag = "1")]
    pub common_tls_context: ::core::option::Option<CommonTlsContext>,
    /// SNI string to use when creating TLS backend connections.
    #[prost(string, tag = "2")]
    pub sni: ::prost::alloc::string::String,
    /// If true, server-initiated TLS renegotiation will be allowed.
    ///
    /// .. attention::
    ///
    /// TLS renegotiation is considered insecure and shouldn't be used unless absolutely necessary.
    #[prost(bool, tag = "3")]
    pub allow_renegotiation: bool,
    /// Maximum number of session keys (Pre-Shared Keys for TLSv1.3+, Session IDs and Session Tickets
    /// for TLSv1.2 and older) to store for the purpose of session resumption.
    ///
    /// Defaults to 1, setting this to 0 disables session resumption.
    #[prost(message, optional, tag = "4")]
    pub max_session_keys: ::core::option::Option<
        super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// This field is used to control the enforcement, whereby the handshake will fail if the keyUsage extension
    /// is present and incompatible with the TLS usage. Currently, the default value is false (i.e., enforcement off)
    /// but it is expected to be changed to true by default in a future release.
    /// `ssl.was_key_usage_invalid` in :ref:`listener metrics <config_listener_stats>` will be set for certificate
    /// configurations that would fail if this option were set to true.
    #[prost(message, optional, tag = "5")]
    pub enforce_rsa_key_usage: ::core::option::Option<
        super::super::super::super::super::google::protobuf::BoolValue,
    >,
}
/// \[\#next-free-field: 11\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownstreamTlsContext {
    /// Common TLS context settings.
    #[prost(message, optional, tag = "1")]
    pub common_tls_context: ::core::option::Option<CommonTlsContext>,
    /// If specified, Envoy will reject connections without a valid client
    /// certificate.
    #[prost(message, optional, tag = "2")]
    pub require_client_certificate: ::core::option::Option<
        super::super::super::super::super::google::protobuf::BoolValue,
    >,
    /// If specified, Envoy will reject connections without a valid and matching SNI.
    /// \[\#not-implemented-hide:\]
    #[prost(message, optional, tag = "3")]
    pub require_sni: ::core::option::Option<
        super::super::super::super::super::google::protobuf::BoolValue,
    >,
    /// If set to true, the TLS server will not maintain a session cache of TLS sessions. (This is
    /// relevant only for TLSv1.2 and earlier.)
    #[prost(bool, tag = "10")]
    pub disable_stateful_session_resumption: bool,
    /// If specified, `session_timeout` will change the maximum lifetime (in seconds) of the TLS session.
    /// Currently this value is used as a hint for the `TLS session ticket lifetime (for TLSv1.2) <<https://tools.ietf.org/html/rfc5077#section-5.6>`\_.>
    /// Only seconds can be specified (fractional seconds are ignored).
    #[prost(message, optional, tag = "6")]
    pub session_timeout: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Duration,
    >,
    /// Config for whether to use certificates if they do not have
    /// an accompanying OCSP response or if the response expires at runtime.
    /// Defaults to LENIENT_STAPLING
    #[prost(enumeration = "downstream_tls_context::OcspStaplePolicy", tag = "8")]
    pub ocsp_staple_policy: i32,
    /// Multiple certificates are allowed in Downstream transport socket to serve different SNI.
    /// If the client provides SNI but no such cert matched, it will decide to full scan certificates or not based on this config.
    /// Defaults to false. See more details in :ref:`Multiple TLS certificates <arch_overview_ssl_cert_select>`.
    #[prost(message, optional, tag = "9")]
    pub full_scan_certs_on_sni_mismatch: ::core::option::Option<
        super::super::super::super::super::google::protobuf::BoolValue,
    >,
    #[prost(oneof = "downstream_tls_context::SessionTicketKeysType", tags = "4, 5, 7")]
    pub session_ticket_keys_type: ::core::option::Option<
        downstream_tls_context::SessionTicketKeysType,
    >,
}
/// Nested message and enum types in `DownstreamTlsContext`.
pub mod downstream_tls_context {
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
    pub enum OcspStaplePolicy {
        /// OCSP responses are optional. If an OCSP response is absent
        /// or expired, the associated certificate will be used for
        /// connections without an OCSP staple.
        LenientStapling = 0,
        /// OCSP responses are optional. If an OCSP response is absent,
        /// the associated certificate will be used without an
        /// OCSP staple. If a response is provided but is expired,
        /// the associated certificate will not be used for
        /// subsequent connections. If no suitable certificate is found,
        /// the connection is rejected.
        StrictStapling = 1,
        /// OCSP responses are required. Configuration will fail if
        /// a certificate is provided without an OCSP response. If a
        /// response expires, the associated certificate will not be
        /// used connections. If no suitable certificate is found, the
        /// connection is rejected.
        MustStaple = 2,
    }
    impl OcspStaplePolicy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OcspStaplePolicy::LenientStapling => "LENIENT_STAPLING",
                OcspStaplePolicy::StrictStapling => "STRICT_STAPLING",
                OcspStaplePolicy::MustStaple => "MUST_STAPLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LENIENT_STAPLING" => Some(Self::LenientStapling),
                "STRICT_STAPLING" => Some(Self::StrictStapling),
                "MUST_STAPLE" => Some(Self::MustStaple),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SessionTicketKeysType {
        /// TLS session ticket key settings.
        #[prost(message, tag = "4")]
        SessionTicketKeys(super::TlsSessionTicketKeys),
        /// Config for fetching TLS session ticket keys via SDS API.
        #[prost(message, tag = "5")]
        SessionTicketKeysSdsSecretConfig(super::SdsSecretConfig),
        /// Config for controlling stateless TLS session resumption: setting this to true will cause the TLS
        /// server to not issue TLS session tickets for the purposes of stateless TLS session resumption.
        /// If set to false, the TLS server will issue TLS session tickets and encrypt/decrypt them using
        /// the keys specified through either :ref:`session_ticket_keys <envoy_v3_api_field_extensions.transport_sockets.tls.v3.DownstreamTlsContext.session_ticket_keys>`
        /// or :ref:`session_ticket_keys_sds_secret_config <envoy_v3_api_field_extensions.transport_sockets.tls.v3.DownstreamTlsContext.session_ticket_keys_sds_secret_config>`.
        /// If this config is set to false and no keys are explicitly configured, the TLS server will issue
        /// TLS session tickets and encrypt/decrypt them using an internally-generated and managed key, with the
        /// implication that sessions cannot be resumed across hot restarts or on different hosts.
        #[prost(bool, tag = "7")]
        DisableStatelessSessionResumption(bool),
    }
}
/// TLS key log configuration.
/// The key log file format is "format used by NSS for its SSLKEYLOGFILE debugging output" (text taken from openssl man page)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TlsKeyLog {
    /// The path to save the TLS key log.
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// The local IP address that will be used to filter the connection which should save the TLS key log
    /// If it is not set, any local IP address  will be matched.
    #[prost(message, repeated, tag = "2")]
    pub local_address_range: ::prost::alloc::vec::Vec<
        super::super::super::super::config::core::v3::CidrRange,
    >,
    /// The remote IP address that will be used to filter the connection which should save the TLS key log
    /// If it is not set, any remote IP address will be matched.
    #[prost(message, repeated, tag = "3")]
    pub remote_address_range: ::prost::alloc::vec::Vec<
        super::super::super::super::config::core::v3::CidrRange,
    >,
}
/// TLS context shared by both client and server TLS contexts.
/// \[\#next-free-field: 16\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonTlsContext {
    /// TLS protocol versions, cipher suites etc.
    #[prost(message, optional, tag = "1")]
    pub tls_params: ::core::option::Option<TlsParameters>,
    /// Only a single TLS certificate is supported in client contexts. In server contexts,
    /// :ref:`Multiple TLS certificates <arch_overview_ssl_cert_select>` can be associated with the
    /// same context to allow both RSA and ECDSA certificates and support SNI-based selection.
    ///
    /// Only one of `tls_certificates`, `tls_certificate_sds_secret_configs`,
    /// and `tls_certificate_provider_instance` may be used.
    /// \[\#next-major-version: These mutually exclusive fields should ideally be in a oneof, but it's
    /// not legal to put a repeated field in a oneof. In the next major version, we should rework
    /// this to avoid this problem.\]
    #[prost(message, repeated, tag = "2")]
    pub tls_certificates: ::prost::alloc::vec::Vec<TlsCertificate>,
    /// Configs for fetching TLS certificates via SDS API. Note SDS API allows certificates to be
    /// fetched/refreshed over the network asynchronously with respect to the TLS handshake.
    ///
    /// The same number and types of certificates as :ref:`tls_certificates <envoy_v3_api_field_extensions.transport_sockets.tls.v3.CommonTlsContext.tls_certificates>`
    /// are valid in the the certificates fetched through this setting.
    ///
    /// Only one of `tls_certificates`, `tls_certificate_sds_secret_configs`,
    /// and `tls_certificate_provider_instance` may be used.
    /// \[\#next-major-version: These mutually exclusive fields should ideally be in a oneof, but it's
    /// not legal to put a repeated field in a oneof. In the next major version, we should rework
    /// this to avoid this problem.\]
    #[prost(message, repeated, tag = "6")]
    pub tls_certificate_sds_secret_configs: ::prost::alloc::vec::Vec<SdsSecretConfig>,
    /// Certificate provider instance for fetching TLS certs.
    ///
    /// Only one of `tls_certificates`, `tls_certificate_sds_secret_configs`,
    /// and `tls_certificate_provider_instance` may be used.
    /// \[\#not-implemented-hide:\]
    #[prost(message, optional, tag = "14")]
    pub tls_certificate_provider_instance: ::core::option::Option<
        CertificateProviderPluginInstance,
    >,
    /// Certificate provider for fetching TLS certificates.
    /// \[\#not-implemented-hide:\]
    #[deprecated]
    #[prost(message, optional, tag = "9")]
    pub tls_certificate_certificate_provider: ::core::option::Option<
        common_tls_context::CertificateProvider,
    >,
    /// Certificate provider instance for fetching TLS certificates.
    /// \[\#not-implemented-hide:\]
    #[deprecated]
    #[prost(message, optional, tag = "11")]
    pub tls_certificate_certificate_provider_instance: ::core::option::Option<
        common_tls_context::CertificateProviderInstance,
    >,
    /// Supplies the list of ALPN protocols that the listener should expose. In
    /// practice this is likely to be set to one of two values (see the
    /// :ref:`codec_type <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.codec_type>`
    /// parameter in the HTTP connection manager for more information):
    ///
    /// * "h2,http/1.1" If the listener is going to support both HTTP/2 and HTTP/1.1.
    /// * "http/1.1" If the listener is only going to support HTTP/1.1.
    ///
    /// There is no default for this parameter. If empty, Envoy will not expose ALPN.
    #[prost(string, repeated, tag = "4")]
    pub alpn_protocols: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Custom TLS handshaker. If empty, defaults to native TLS handshaking
    /// behavior.
    #[prost(message, optional, tag = "13")]
    pub custom_handshaker: ::core::option::Option<
        super::super::super::super::config::core::v3::TypedExtensionConfig,
    >,
    /// TLS key log configuration
    #[prost(message, optional, tag = "15")]
    pub key_log: ::core::option::Option<TlsKeyLog>,
    #[prost(
        oneof = "common_tls_context::ValidationContextType",
        tags = "3, 7, 8, 10, 12"
    )]
    pub validation_context_type: ::core::option::Option<
        common_tls_context::ValidationContextType,
    >,
}
/// Nested message and enum types in `CommonTlsContext`.
pub mod common_tls_context {
    /// Config for Certificate provider to get certificates. This provider should allow certificates to be
    /// fetched/refreshed over the network asynchronously with respect to the TLS handshake.
    ///
    /// DEPRECATED: This message is not currently used, but if we ever do need it, we will want to
    /// move it out of CommonTlsContext and into common.proto, similar to the existing
    /// CertificateProviderPluginInstance message.
    ///
    /// \[\#not-implemented-hide:\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CertificateProvider {
        /// opaque name used to specify certificate instances or types. For example, "ROOTCA" to specify
        /// a root-certificate (validation context) or "TLS" to specify a new tls-certificate.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Provider specific config.
        /// Note: an implementation is expected to dedup multiple instances of the same config
        /// to maintain a single certificate-provider instance. The sharing can happen, for
        /// example, among multiple clusters or between the tls_certificate and validation_context
        /// certificate providers of a cluster.
        /// This config could be supplied inline or (in future) a named xDS resource.
        #[prost(oneof = "certificate_provider::Config", tags = "2")]
        pub config: ::core::option::Option<certificate_provider::Config>,
    }
    /// Nested message and enum types in `CertificateProvider`.
    pub mod certificate_provider {
        /// Provider specific config.
        /// Note: an implementation is expected to dedup multiple instances of the same config
        /// to maintain a single certificate-provider instance. The sharing can happen, for
        /// example, among multiple clusters or between the tls_certificate and validation_context
        /// certificate providers of a cluster.
        /// This config could be supplied inline or (in future) a named xDS resource.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Config {
            #[prost(message, tag = "2")]
            TypedConfig(
                super::super::super::super::super::super::config::core::v3::TypedExtensionConfig,
            ),
        }
    }
    /// Similar to CertificateProvider above, but allows the provider instances to be configured on
    /// the client side instead of being sent from the control plane.
    ///
    /// DEPRECATED: This message was moved outside of CommonTlsContext
    /// and now lives in common.proto.
    ///
    /// \[\#not-implemented-hide:\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CertificateProviderInstance {
        /// Provider instance name. This name must be defined in the client's configuration (e.g., a
        /// bootstrap file) to correspond to a provider instance (i.e., the same data in the typed_config
        /// field that would be sent in the CertificateProvider message if the config was sent by the
        /// control plane). If not present, defaults to "default".
        ///
        /// Instance names should generally be defined not in terms of the underlying provider
        /// implementation (e.g., "file_watcher") but rather in terms of the function of the
        /// certificates (e.g., "foo_deployment_identity").
        #[prost(string, tag = "1")]
        pub instance_name: ::prost::alloc::string::String,
        /// Opaque name used to specify certificate instances or types. For example, "ROOTCA" to specify
        /// a root-certificate (validation context) or "example.com" to specify a certificate for a
        /// particular domain. Not all provider instances will actually use this field, so the value
        /// defaults to the empty string.
        #[prost(string, tag = "2")]
        pub certificate_name: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CombinedCertificateValidationContext {
        /// How to validate peer certificates.
        #[prost(message, optional, tag = "1")]
        pub default_validation_context: ::core::option::Option<
            super::CertificateValidationContext,
        >,
        /// Config for fetching validation context via SDS API. Note SDS API allows certificates to be
        /// fetched/refreshed over the network asynchronously with respect to the TLS handshake.
        #[prost(message, optional, tag = "2")]
        pub validation_context_sds_secret_config: ::core::option::Option<
            super::SdsSecretConfig,
        >,
        /// Certificate provider for fetching CA certs. This will populate the
        /// `default_validation_context.trusted_ca` field.
        /// \[\#not-implemented-hide:\]
        #[deprecated]
        #[prost(message, optional, tag = "3")]
        pub validation_context_certificate_provider: ::core::option::Option<
            CertificateProvider,
        >,
        /// Certificate provider instance for fetching CA certs. This will populate the
        /// `default_validation_context.trusted_ca` field.
        /// \[\#not-implemented-hide:\]
        #[deprecated]
        #[prost(message, optional, tag = "4")]
        pub validation_context_certificate_provider_instance: ::core::option::Option<
            CertificateProviderInstance,
        >,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ValidationContextType {
        /// How to validate peer certificates.
        #[prost(message, tag = "3")]
        ValidationContext(super::CertificateValidationContext),
        /// Config for fetching validation context via SDS API. Note SDS API allows certificates to be
        /// fetched/refreshed over the network asynchronously with respect to the TLS handshake.
        #[prost(message, tag = "7")]
        ValidationContextSdsSecretConfig(super::SdsSecretConfig),
        /// Combined certificate validation context holds a default CertificateValidationContext
        /// and SDS config. When SDS server returns dynamic CertificateValidationContext, both dynamic
        /// and default CertificateValidationContext are merged into a new CertificateValidationContext
        /// for validation. This merge is done by Message::MergeFrom(), so dynamic
        /// CertificateValidationContext overwrites singular fields in default
        /// CertificateValidationContext, and concatenates repeated fields to default
        /// CertificateValidationContext, and logical OR is applied to boolean fields.
        #[prost(message, tag = "8")]
        CombinedValidationContext(CombinedCertificateValidationContext),
        /// Certificate provider for fetching validation context.
        /// \[\#not-implemented-hide:\]
        #[prost(message, tag = "10")]
        ValidationContextCertificateProvider(CertificateProvider),
        /// Certificate provider instance for fetching validation context.
        /// \[\#not-implemented-hide:\]
        #[prost(message, tag = "12")]
        ValidationContextCertificateProviderInstance(CertificateProviderInstance),
    }
}
/// Configuration specific to the `SPIFFE <<https://github.com/spiffe/spiffe>`\_> certificate validator.
///
/// Example:
///
/// .. validated-code-block:: yaml
/// :type-name: envoy.extensions.transport_sockets.tls.v3.CertificateValidationContext
///
/// custom_validator_config:
/// name: envoy.tls.cert_validator.spiffe
/// typed_config:
/// "@type": type.googleapis.com/envoy.extensions.transport_sockets.tls.v3.SPIFFECertValidatorConfig
/// trust_domains:
/// - name: foo.com
/// trust_bundle:
/// filename: "foo.pem"
/// - name: envoy.com
/// trust_bundle:
/// filename: "envoy.pem"
///
/// In this example, a presented peer certificate whose SAN matches `spiffe://foo.com/**` is validated against
/// the "foo.pem" x.509 certificate. All the trust bundles are isolated from each other, so no trust domain can mint
/// a SVID belonging to another trust domain. That means, in this example, a SVID signed by `envoy.com`'s CA with `spiffe://foo.com/**`
/// SAN would be rejected since Envoy selects the trust bundle according to the presented SAN before validate the certificate.
///
/// Note that SPIFFE validator inherits and uses the following options from :ref:`CertificateValidationContext <envoy_v3_api_msg_extensions.transport_sockets.tls.v3.CertificateValidationContext>`.
///
/// * :ref:`allow_expired_certificate <envoy_v3_api_field_extensions.transport_sockets.tls.v3.CertificateValidationContext.allow_expired_certificate>` to allow expired certificates.
/// * :ref:`match_typed_subject_alt_names <envoy_v3_api_field_extensions.transport_sockets.tls.v3.CertificateValidationContext.match_typed_subject_alt_names>` to match **URI** SAN of certificates. Unlike the default validator, SPIFFE validator only matches **URI** SAN (which equals to SVID in SPIFFE terminology) and ignore other SAN types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpiffeCertValidatorConfig {
    /// This field specifies trust domains used for validating incoming X.509-SVID(s).
    #[prost(message, repeated, tag = "1")]
    pub trust_domains: ::prost::alloc::vec::Vec<
        spiffe_cert_validator_config::TrustDomain,
    >,
}
/// Nested message and enum types in `SPIFFECertValidatorConfig`.
pub mod spiffe_cert_validator_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TrustDomain {
        /// Name of the trust domain, `example.com`, `foo.bar.gov` for example.
        /// Note that this must *not* have "spiffe://" prefix.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Specify a data source holding x.509 trust bundle used for validating incoming SVID(s) in this trust domain.
        #[prost(message, optional, tag = "2")]
        pub trust_bundle: ::core::option::Option<
            super::super::super::super::super::config::core::v3::DataSource,
        >,
    }
}
