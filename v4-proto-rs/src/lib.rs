/// re-export of cosmos-sdk
pub use cosmos_sdk_proto;

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/_includes.rs"));

use prost::Message;
use prost_types;

macro_rules! impl_prost_any_from {
    ($type:ty) => {
        impl From<$type> for prost_types::Any {
            fn from(msg: $type) -> Self {
                let value = msg.encode_to_vec();
                let type_url = stringify!($type)
                    .replace("crate::", "/")
                    .replace("::", ".");
                prost_types::Any {
                    type_url,
                    value,
                }
            }
        }
    };
}

impl_prost_any_from!(crate::dydxprotocol::clob::MsgPlaceOrder);
impl_prost_any_from!(crate::dydxprotocol::clob::MsgCancelOrder);
impl_prost_any_from!(crate::dydxprotocol::sending::MsgCreateTransfer);
impl_prost_any_from!(crate::dydxprotocol::sending::MsgDepositToSubaccount);
impl_prost_any_from!(crate::dydxprotocol::sending::MsgWithdrawFromSubaccount);
