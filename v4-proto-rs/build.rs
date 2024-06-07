use prost_build::Config;
use std::env;
use std::path::PathBuf;

fn main() -> Result<(), tonic_buf_build::error::TonicBufBuildError> {
    if std::env::var("V4_PROTO_REBUILD").is_ok() {
        let mut config = Config::new();
        config.out_dir("src");
        config.include_file("_includes.rs");
        let mut path = PathBuf::from(
            env::var("CARGO_MANIFEST_DIR").expect("cargo sets CARGO_MANIFEST_DIR env var"),
        );
        path.pop();
        tonic_buf_build::compile_from_buf_workspace_with_config(
            tonic_build::configure().build_server(false),
            Some(config),
            tonic_buf_build::TonicBufConfig {
                buf_dir: Some(path),
            },
        )?;
    }

    Ok(())
}
