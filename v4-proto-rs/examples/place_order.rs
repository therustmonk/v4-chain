use anyhow::{anyhow, Result};
use chrono::{TimeDelta, Utc};
use cosmrs::{
    tx::{self, Fee, SignDoc, SignerInfo},
    Coin,
};
use rand::{thread_rng, Rng};
use std::env;
use std::str::FromStr;
use tonic::{
    transport::{Channel, ClientTlsConfig},
    Code, Request,
};
use tower::timeout::Timeout;
use v4_proto_rs::cosmos_sdk_proto::{
    cosmos::auth::v1beta1::{
        query_client::QueryClient as AuthGrpcClient, BaseAccount, QueryAccountRequest,
    },
    cosmos::tx::v1beta1::{
        service_client::ServiceClient as TxGrpcClient, BroadcastMode, BroadcastTxRequest,
        GetTxRequest,
    },
    traits::Message,
};
use v4_proto_rs::dydxprotocol::clob::{
    order::{ConditionType, GoodTilOneof, Side, TimeInForce},
    MsgPlaceOrder, Order, OrderId,
};
use v4_proto_rs::dydxprotocol::subaccounts::SubaccountId;

const TIMEOUT_MILLIS: u64 = 1000;
const ETH_USD_PAIR_ID: u32 = 1; // information on market id can be fetch from indexer API
const ETH_USD_QUANTUMS: u64 = 10_000_000; // calculated based on market
const SUBTICKS: u64 = 40_000_000_000; // calculated based on market and price
const ORDER_FLAGS_LONG_TERM: u32 = 64; // for short term order is 32
const CHAIN_ID: &str = "dydx-testnet-4"; // https://docs.dydx.exchange/infrastructure_providers-network/network_constants
const BECH32_PREFIX: &str = "dydx"; // account prefix https://docs.cosmos.network/main/learn/beginner/accounts
const DEFAULT_QUERY_TIMEOUT_SECS: u64 = 15;
const DEFAULT_QUERY_INTERVAL_SECS: u64 = 2;
const DYDX_FEE_DENOM: &str = "afet";
const TEST_MNEMONIC: &str = "mirror actor skill push coach wait confirm orchard lunch mobile athlete gossip awake miracle matter bus reopen team ladder lazy list timber render wait";

struct CosmosClient {
    auth: AuthGrpcClient<Timeout<Channel>>,
    tx: TxGrpcClient<Timeout<Channel>>,
}

impl CosmosClient {
    async fn connect(endpoint: String) -> Result<Self> {
        // WARNING: this example uses root certificates of OS
        // which may be vulnerable under malware attacks
        // for production deployments you should better specify a ceritificate
        let tls = ClientTlsConfig::new();

        // WARNING: for production use consider load-balancing
        // see https://github.com/hyperium/tonic/blob/master/examples/src/dynamic_load_balance/client.rs
        let channel = Channel::from_shared(endpoint.clone())
            .map_err(|s| anyhow!("channel creation error for {endpoint}: {s}"))?
            .tls_config(tls)?
            .connect()
            .await
            .map_err(|s| anyhow!("connection error to {endpoint}: {s}"))?;
        let timeout_channel =
            Timeout::new(channel, std::time::Duration::from_millis(TIMEOUT_MILLIS));
        Ok(Self {
            auth: AuthGrpcClient::new(timeout_channel.clone()),
            tx: TxGrpcClient::new(timeout_channel),
        })
    }
}

fn prepare_order(subaccount: u32, address: String) -> Result<MsgPlaceOrder> {
    let now = Utc::now();
    let time_in_force_seconds = now + TimeDelta::seconds(60);
    let order = Order {
        order_id: Some(OrderId {
            subaccount_id: Some(SubaccountId {
                owner: address,
                number: subaccount,
            }),
            // The client ID of this order, unique with respect to the specific
            // sub account. Can be used in future orders (for cancel, replace etc)
            client_id: thread_rng().gen_range(0..100_000_000),
            order_flags: ORDER_FLAGS_LONG_TERM,
            clob_pair_id: ETH_USD_PAIR_ID,
        }),
        // Buy or Sell?
        side: Side::Buy.into(),
        // The size of this order in base quantums. Must be a multiple of
        // https://docs.dydx.exchange/api_integration-guides/how_to_interpret_block_data_for_trades#quantums
        quantums: ETH_USD_QUANTUMS,
        // The price level that this order will be placed at on the orderbook,
        // in subticks
        // https://docs.dydx.exchange/api_integration-guides/how_to_interpret_block_data_for_trades#subticks
        subticks: SUBTICKS,
        // TimeInForce indicates how long an order will remain active before it is executed or expires
        // TIME_IN_FORCE_UNSPECIFIED is the same as GTT Good Til Time
        time_in_force: TimeInForce::Unspecified.into(),
        // Enforces that the order can only reduce the size of an existing position.
        reduce_only: false,
        // Set of bit flags set arbitrarily by clients and ignored by the protocol.
        // Used by indexer to infer information about a placed order.
        client_metadata: 0u32,
        // Unconditional order
        condition_type: ConditionType::Unspecified.into(),
        // conditional_order_trigger_subticks represents the price at which this order
        // will be triggered
        conditional_order_trigger_subticks: 0u64,
        // Information about when the order expires.
        // We use Long Term Order for our example which expires in 60 secs
        good_til_oneof: Some(GoodTilOneof::GoodTilBlockTime(
            time_in_force_seconds.timestamp().try_into()?,
        )),
    };
    Ok(MsgPlaceOrder { order: Some(order) })
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let endpoint = args
        .get(1)
        .expect("testnet node grpc endpoint should be provided as a first argument");
    // Connecting to dYdX testnet
    let mut client = CosmosClient::connect(endpoint.to_string()).await?;

    // Obtaining keys
    let mnemonic_seed = bip32::Mnemonic::new(TEST_MNEMONIC, bip32::Language::English)?.to_seed("");
    let subaccount = 0;
    // https://github.com/satoshilabs/slips/blob/master/slip-0044.md
    let derivation_path =
        bip32::DerivationPath::from_str(&format!("m/44'/118'/0'/0/{subaccount}",))?;
    // WARNING: for production use it is worth to consider
    // https://github.com/rust-bitcoin/rust-secp256k1
    // However, cosmrs uses unaudited crate k256
    let private_key =
        cosmrs::crypto::secp256k1::SigningKey::derive_from_path(mnemonic_seed, &derivation_path)?;
    let public_key = private_key.public_key();
    let accound_id = public_key.account_id(BECH32_PREFIX)?;
    let address = accound_id.to_string();

    // Fetching account sequence number
    let response = client
        .auth
        .account(Request::new(QueryAccountRequest {
            address: address.clone(),
        }))
        .await?
        .into_inner();
    let (account_number, sequence_number) =
        BaseAccount::decode(&response.account.expect("account should exist").value[..])
            .map(|res| (res.account_number, res.sequence))?;

    // Prepare transaction body message
    let msg = prepare_order(subaccount, address)?;

    // Build and sign transaction
    let tx_body = tx::BodyBuilder::new().msg(msg).memo("").finish();
    let amount = Coin {
        amount: 0u8.into(),
        denom: DYDX_FEE_DENOM.parse()?,
    };
    let gas = 0u64; // or gas can be estimated via transaction simulation
    let fee = Fee::from_amount_and_gas(amount, gas);
    let auth_info = SignerInfo::single_direct(Some(public_key), sequence_number).auth_info(fee);
    let chain_id = CHAIN_ID.parse()?;
    let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, account_number)?;
    let tx_raw = sign_doc.sign(&private_key)?;

    // Broadcasting transaction
    let request = BroadcastTxRequest {
        tx_bytes: tx_raw.to_bytes()?,
        mode: BroadcastMode::Sync.into(),
    };

    let txhash = client
        .tx
        .broadcast_tx(request)
        .await?
        .into_inner()
        .tx_response
        .expect("tx should be present")
        .txhash;
    println!("Tx hash: {txhash}");

    // Quering network for transaction
    let attempts = DEFAULT_QUERY_TIMEOUT_SECS / DEFAULT_QUERY_INTERVAL_SECS;
    for _ in 0..attempts {
        match client
            .tx
            .get_tx(GetTxRequest {
                hash: txhash.clone(),
            })
            .await
        {
            Ok(r) => {
                let response = r.into_inner().tx_response.expect("tx should be present");
                println!("Order placement response:\n{response:#?}");
                break;
            }
            Err(status) if status.code() == Code::NotFound => {
                tokio::time::sleep(tokio::time::Duration::from_secs(
                    DEFAULT_QUERY_INTERVAL_SECS,
                ))
                .await;
                continue;
            }
            Err(status) => {
                panic!("error quering tx {txhash}: {status}");
            }
        }
    }
    Ok(())
}
