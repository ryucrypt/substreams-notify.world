// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogMineEvents {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<LogMineEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogMineEvent {
    /// transaction
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub action_ordinal: u32,
    #[prost(message, optional, tag="3")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    /// data payload
    #[prost(string, tag="4")]
    pub miner: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub bounty: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub landowner: ::prost::alloc::string::String,
    #[prost(uint64, tag="7")]
    pub land_id: u64,
    #[prost(string, tag="8")]
    pub landowner_share: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
