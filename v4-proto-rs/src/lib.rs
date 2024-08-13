/// re-export of cosmos-sdk
pub use cosmos_sdk_proto;

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/_includes.rs"));

use prost::Message;
use prost_types;
use std::any::type_name;

pub trait ToAny: Message + Sized {
    fn to_any(self) -> prost_types::Any {
        let value = self.encode_to_vec();
        let type_url = type_name::<Self>()
            .replace("v4_proto_rs::", "/")
            .replace("cosmos_sdk_proto::", "/")
            .replace("::", ".")
            .to_string();
        prost_types::Any {
            type_url,
            value,
        }
    }
}

impl<M: Message> ToAny for M {}

#[cfg(test)]
mod test {
    use super::ToAny;
    use crate::dydxprotocol::clob::MsgCancelOrder;
    use crate::cosmos_sdk_proto::cosmos::bank::v1beta1::MsgSend;

    #[test]
    pub fn test_any_conversion() {
        let msg = MsgCancelOrder {
            order_id: None,
            good_til_oneof: None,
        };
        let any = msg.to_any();
        let url = "/dydxprotocol.clob.MsgCancelOrder";
        assert_eq!(any.type_url, url);
    }

    #[test]
    pub fn test_any_conversion_wrapped() {
        let msg = MsgSend::default();
        let any = msg.to_any();
        let url = "/cosmos.bank.v1beta1.MsgSend";
        assert_eq!(any.type_url, url);
    }
}
