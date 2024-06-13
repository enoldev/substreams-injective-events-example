// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MyEvent {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MyEventList {
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<MyEvent>,
}
// @@protoc_insertion_point(module)
