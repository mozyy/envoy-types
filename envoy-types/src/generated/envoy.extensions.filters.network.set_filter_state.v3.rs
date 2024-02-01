#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    /// A sequence of the filter state values to apply in the specified order
    /// when a new connection is received.
    #[prost(message, repeated, tag = "1")]
    pub on_new_connection: ::prost::alloc::vec::Vec<
        super::super::super::common::set_filter_state::v3::FilterStateValue,
    >,
}
