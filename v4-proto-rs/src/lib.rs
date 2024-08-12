/// re-export of cosmos-sdk
pub use cosmos_sdk_proto;

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/_includes.rs"));

use const_str::replace;
use prost::Message;
use prost_types;

macro_rules! impl_prost_any_from {
    ($type:ty) => {
        impl From<$type> for prost_types::Any {
            fn from(msg: $type) -> Self {
                let value = msg.encode_to_vec();
                const TYPE: &str = stringify!($type);
                const TYPE_WITH_ROOT: &str = replace!(TYPE, "crate::", "/");
                const TYPE_AS_PATH: &str = replace!(TYPE_WITH_ROOT, "::", ".");
                prost_types::Any {
                    type_url: TYPE_AS_PATH.to_string(),
                    value,
                }
            }
        }
    };
}

impl_prost_any_from!(crate::dydxprotocol::clob::MsgPlaceOrder);
impl_prost_any_from!(crate::dydxprotocol::clob::MsgCancelOrder);
impl_prost_any_from!(crate::dydxprotocol::clob::MsgBatchCancel);
impl_prost_any_from!(crate::dydxprotocol::sending::MsgCreateTransfer);
impl_prost_any_from!(crate::dydxprotocol::sending::MsgDepositToSubaccount);
impl_prost_any_from!(crate::dydxprotocol::sending::MsgWithdrawFromSubaccount);

macro_rules! impl_prost_any_from_wrapped {
    ($type:ty, $wrapped:ident) => {
        pub struct $wrapped(pub $type);
        impl From<$wrapped> for prost_types::Any {
            fn from(wmsg: $wrapped) -> Self {
                let value = wmsg.0.encode_to_vec();
                const TYPE: &str = stringify!($type);
                const TYPE_WITH_ROOT: &str = replace!(TYPE, "crate::cosmos_sdk_proto::", "/");
                const TYPE_AS_PATH: &str = replace!(TYPE_WITH_ROOT, "::", ".");
                prost_types::Any {
                    type_url: TYPE_AS_PATH.to_string(),
                    value,
                }
            }
        }
    }
}

pub mod wrapped {
    use const_str::replace;
    use prost::Message;
    impl_prost_any_from_wrapped!(crate::cosmos_sdk_proto::cosmos::bank::v1beta1::MsgSend, WrappedMsgSend);
}

#[cfg(test)]
mod test {
    use crate::dydxprotocol::clob::MsgCancelOrder;
    use prost_types::Any;
    use crate::cosmos_sdk_proto::cosmos::bank::v1beta1::MsgSend;
    use crate::wrapped::WrappedMsgSend;

    #[test]
    pub fn test_any_conversion() {
        let msg = MsgCancelOrder {
            order_id: None,
            good_til_oneof: None,
        };
        let any = Any::from(msg);
        let url = "/dydxprotocol.clob.MsgCancelOrder";
        assert_eq!(any.type_url, url);
    }

    #[test]
    pub fn test_any_conversion_wrapped() {
        let msg = MsgSend::default();
        let any = Any::from(WrappedMsgSend { 0: msg });
        let url = "/cosmos.bank.v1beta1.MsgSend";
        assert_eq!(any.type_url, url);
    }
}
