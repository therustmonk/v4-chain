fn main() {
    prost_build::compile_protos(
        &["proto/dydxprotocol/assets/genesis.proto"],
        &["proto/.proto-export", "proto/dydxprotocol"],
    )
    .unwrap();
}
