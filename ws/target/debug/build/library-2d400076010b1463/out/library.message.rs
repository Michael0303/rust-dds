/// A message test
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Msg {
    #[prost(string, tag="1")]
    pub name: std::string::String,
    #[prost(int32, tag="2")]
    pub id: i32,
    #[prost(double, tag="3")]
    pub testdata: f64,
    #[prost(string, tag="4")]
    pub from: std::string::String,
}
pub mod msg {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PlayerType {
        Goalkeeper = 0,
        Striker = 1,
        Defender = 2,
    }
}
