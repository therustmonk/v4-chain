/// re-export of cosmos-sdk
pub use cosmos_sdk_proto;

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/_includes.rs"));

use crate::dydxprotocol::clob::MsgPlaceOrder;
use prost::Message;

impl Into<prost_types::Any> for MsgPlaceOrder {
    fn into(self) -> prost_types::Any {
        let value = self.encode_to_vec();
        prost_types::Any {
            type_url: "/dydxprotocol.clob.MsgPlaceOrder".to_string(),
            value,
        }
    }
}
