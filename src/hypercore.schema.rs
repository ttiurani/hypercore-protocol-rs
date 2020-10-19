// FIXME: Bazel build to generate this file in build.rs, so let's just save it to src/ for now

/// Sent as part of the noise protocol.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoisePayload {
    #[prost(bytes, required, tag = "1")]
    pub nonce: std::vec::Vec<u8>,
}
/// type=0
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Open {
    #[prost(bytes, required, tag = "1")]
    pub discovery_key: std::vec::Vec<u8>,
    #[prost(bytes, optional, tag = "2")]
    pub capability: ::std::option::Option<std::vec::Vec<u8>>,
}
/// type=1, overall feed options. can be sent multiple times
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Options {
    /// Should be sorted lexicographically
    #[prost(string, repeated, tag = "1")]
    pub extensions: ::std::vec::Vec<std::string::String>,
    /// Should all blocks be explicitly acknowledged?
    #[prost(bool, optional, tag = "2")]
    pub ack: ::std::option::Option<bool>,
}
/// type=2, message indicating state changes etc.
/// initial state for uploading/downloading is true
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    #[prost(bool, optional, tag = "1")]
    pub uploading: ::std::option::Option<bool>,
    #[prost(bool, optional, tag = "2")]
    pub downloading: ::std::option::Option<bool>,
}
/// type=3, what do we have?
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Have {
    #[prost(uint64, required, tag = "1")]
    pub start: u64,
    /// defaults to 1
    #[prost(uint64, optional, tag = "2", default = "1")]
    pub length: ::std::option::Option<u64>,
    #[prost(bytes, optional, tag = "3")]
    pub bitfield: ::std::option::Option<std::vec::Vec<u8>>,
    /// when true, this Have message is an acknowledgement
    #[prost(bool, optional, tag = "4")]
    pub ack: ::std::option::Option<bool>,
}
/// type=4, what did we lose?
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Unhave {
    #[prost(uint64, required, tag = "1")]
    pub start: u64,
    /// defaults to 1
    #[prost(uint64, optional, tag = "2", default = "1")]
    pub length: ::std::option::Option<u64>,
}
/// type=5, what do we want? remote should start sending have messages in this range
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Want {
    #[prost(uint64, required, tag = "1")]
    pub start: u64,
    /// defaults to Infinity or feed.length (if not live)
    #[prost(uint64, optional, tag = "2")]
    pub length: ::std::option::Option<u64>,
}
/// type=6, what don't we want anymore?
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Unwant {
    #[prost(uint64, required, tag = "1")]
    pub start: u64,
    /// defaults to Infinity or feed.length (if not live)
    #[prost(uint64, optional, tag = "2")]
    pub length: ::std::option::Option<u64>,
}
/// type=7, ask for data
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(uint64, required, tag = "1")]
    pub index: u64,
    #[prost(uint64, optional, tag = "2")]
    pub bytes: ::std::option::Option<u64>,
    #[prost(bool, optional, tag = "3")]
    pub hash: ::std::option::Option<bool>,
    #[prost(uint64, optional, tag = "4")]
    pub nodes: ::std::option::Option<u64>,
}
/// type=8, cancel a request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cancel {
    #[prost(uint64, required, tag = "1")]
    pub index: u64,
    #[prost(uint64, optional, tag = "2")]
    pub bytes: ::std::option::Option<u64>,
    #[prost(bool, optional, tag = "3")]
    pub hash: ::std::option::Option<bool>,
}
/// type=9, get some data
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    #[prost(uint64, required, tag = "1")]
    pub index: u64,
    #[prost(bytes, optional, tag = "2")]
    pub value: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(message, repeated, tag = "3")]
    pub nodes: ::std::vec::Vec<data::Node>,
    #[prost(bytes, optional, tag = "4")]
    pub signature: ::std::option::Option<std::vec::Vec<u8>>,
}
pub mod data {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Node {
        #[prost(uint64, required, tag = "1")]
        pub index: u64,
        #[prost(bytes, required, tag = "2")]
        pub hash: std::vec::Vec<u8>,
        #[prost(uint64, required, tag = "3")]
        pub size: u64,
    }
}
/// type=10, explicitly close a channel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Close {
    /// only send this if you did not do an open
    #[prost(bytes, optional, tag = "1")]
    pub discovery_key: ::std::option::Option<std::vec::Vec<u8>>,
}
