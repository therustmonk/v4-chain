// This file is @generated by prost-build.
/// Params defines the parameters for x/stats module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// The desired number of seconds in the look-back window.
    #[prost(message, optional, tag = "1")]
    pub window_duration: ::core::option::Option<::prost_types::Duration>,
}
impl ::prost::Name for Params {
    const NAME: &'static str = "Params";
    const PACKAGE: &'static str = "dydxprotocol.stats";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.stats.Params".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.stats.Params".into()
    }
}
/// GenesisState defines the stats module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// The parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "dydxprotocol.stats";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.stats.GenesisState".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.stats.GenesisState".into()
    }
}
/// BlockStats is used to store stats transiently within the scope of a block.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockStats {
    /// The fills that occured on this block.
    #[prost(message, repeated, tag = "1")]
    pub fills: ::prost::alloc::vec::Vec<block_stats::Fill>,
}
/// Nested message and enum types in `BlockStats`.
pub mod block_stats {
    /// Fill records data about a fill on this block.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Fill {
        /// Taker wallet address
        #[prost(string, tag = "1")]
        pub taker: ::prost::alloc::string::String,
        /// Maker wallet address
        #[prost(string, tag = "2")]
        pub maker: ::prost::alloc::string::String,
        /// Notional USDC filled in quantums
        #[prost(uint64, tag = "3")]
        pub notional: u64,
    }
    impl ::prost::Name for Fill {
        const NAME: &'static str = "Fill";
        const PACKAGE: &'static str = "dydxprotocol.stats";
        fn full_name() -> ::prost::alloc::string::String {
            "dydxprotocol.stats.BlockStats.Fill".into()
        }
        fn type_url() -> ::prost::alloc::string::String {
            "/dydxprotocol.stats.BlockStats.Fill".into()
        }
    }
}
impl ::prost::Name for BlockStats {
    const NAME: &'static str = "BlockStats";
    const PACKAGE: &'static str = "dydxprotocol.stats";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.stats.BlockStats".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.stats.BlockStats".into()
    }
}
/// StatsMetadata stores metadata for the x/stats module
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatsMetadata {
    /// The oldest epoch that is included in the stats. The next epoch to be
    /// removed from the window.
    #[prost(uint32, tag = "1")]
    pub trailing_epoch: u32,
}
impl ::prost::Name for StatsMetadata {
    const NAME: &'static str = "StatsMetadata";
    const PACKAGE: &'static str = "dydxprotocol.stats";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.stats.StatsMetadata".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.stats.StatsMetadata".into()
    }
}
/// EpochStats stores stats for a particular epoch
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpochStats {
    /// Epoch end time
    #[prost(message, optional, tag = "1")]
    pub epoch_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Stats for each user in this epoch. Sorted by user.
    #[prost(message, repeated, tag = "2")]
    pub stats: ::prost::alloc::vec::Vec<epoch_stats::UserWithStats>,
}
/// Nested message and enum types in `EpochStats`.
pub mod epoch_stats {
    /// A user and its associated stats
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UserWithStats {
        #[prost(string, tag = "1")]
        pub user: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub stats: ::core::option::Option<super::UserStats>,
    }
    impl ::prost::Name for UserWithStats {
        const NAME: &'static str = "UserWithStats";
        const PACKAGE: &'static str = "dydxprotocol.stats";
        fn full_name() -> ::prost::alloc::string::String {
            "dydxprotocol.stats.EpochStats.UserWithStats".into()
        }
        fn type_url() -> ::prost::alloc::string::String {
            "/dydxprotocol.stats.EpochStats.UserWithStats".into()
        }
    }
}
impl ::prost::Name for EpochStats {
    const NAME: &'static str = "EpochStats";
    const PACKAGE: &'static str = "dydxprotocol.stats";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.stats.EpochStats".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.stats.EpochStats".into()
    }
}
/// GlobalStats stores global stats
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalStats {
    /// Notional USDC traded in quantums
    #[prost(uint64, tag = "1")]
    pub notional_traded: u64,
}
impl ::prost::Name for GlobalStats {
    const NAME: &'static str = "GlobalStats";
    const PACKAGE: &'static str = "dydxprotocol.stats";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.stats.GlobalStats".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.stats.GlobalStats".into()
    }
}
/// UserStats stores stats for a User
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserStats {
    /// Taker USDC in quantums
    #[prost(uint64, tag = "1")]
    pub taker_notional: u64,
    /// Maker USDC in quantums
    #[prost(uint64, tag = "2")]
    pub maker_notional: u64,
}
impl ::prost::Name for UserStats {
    const NAME: &'static str = "UserStats";
    const PACKAGE: &'static str = "dydxprotocol.stats";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.stats.UserStats".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.stats.UserStats".into()
    }
}
/// QueryParamsRequest is a request type for the Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
impl ::prost::Name for QueryParamsRequest {
    const NAME: &'static str = "QueryParamsRequest";
    const PACKAGE: &'static str = "dydxprotocol.stats";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.stats.QueryParamsRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.stats.QueryParamsRequest".into()
    }
}
/// QueryParamsResponse is a response type for the Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for QueryParamsResponse {
    const NAME: &'static str = "QueryParamsResponse";
    const PACKAGE: &'static str = "dydxprotocol.stats";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.stats.QueryParamsResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.stats.QueryParamsResponse".into()
    }
}
/// QueryStatsMetadataRequest is a request type for the StatsMetadata RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStatsMetadataRequest {}
impl ::prost::Name for QueryStatsMetadataRequest {
    const NAME: &'static str = "QueryStatsMetadataRequest";
    const PACKAGE: &'static str = "dydxprotocol.stats";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.stats.QueryStatsMetadataRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.stats.QueryStatsMetadataRequest".into()
    }
}
/// QueryStatsMetadataResponse is a response type for the StatsMetadata RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStatsMetadataResponse {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<StatsMetadata>,
}
impl ::prost::Name for QueryStatsMetadataResponse {
    const NAME: &'static str = "QueryStatsMetadataResponse";
    const PACKAGE: &'static str = "dydxprotocol.stats";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.stats.QueryStatsMetadataResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.stats.QueryStatsMetadataResponse".into()
    }
}
/// QueryGlobalStatsRequest is a request type for the GlobalStats RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGlobalStatsRequest {}
impl ::prost::Name for QueryGlobalStatsRequest {
    const NAME: &'static str = "QueryGlobalStatsRequest";
    const PACKAGE: &'static str = "dydxprotocol.stats";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.stats.QueryGlobalStatsRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.stats.QueryGlobalStatsRequest".into()
    }
}
/// QueryGlobalStatsResponse is a response type for the GlobalStats RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGlobalStatsResponse {
    #[prost(message, optional, tag = "1")]
    pub stats: ::core::option::Option<GlobalStats>,
}
impl ::prost::Name for QueryGlobalStatsResponse {
    const NAME: &'static str = "QueryGlobalStatsResponse";
    const PACKAGE: &'static str = "dydxprotocol.stats";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.stats.QueryGlobalStatsResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.stats.QueryGlobalStatsResponse".into()
    }
}
/// QueryUserStatsRequest is a request type for the UserStats RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUserStatsRequest {
    #[prost(string, tag = "1")]
    pub user: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryUserStatsRequest {
    const NAME: &'static str = "QueryUserStatsRequest";
    const PACKAGE: &'static str = "dydxprotocol.stats";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.stats.QueryUserStatsRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.stats.QueryUserStatsRequest".into()
    }
}
/// QueryUserStatsResponse is a request type for the UserStats RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUserStatsResponse {
    #[prost(message, optional, tag = "1")]
    pub stats: ::core::option::Option<UserStats>,
}
impl ::prost::Name for QueryUserStatsResponse {
    const NAME: &'static str = "QueryUserStatsResponse";
    const PACKAGE: &'static str = "dydxprotocol.stats";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.stats.QueryUserStatsResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.stats.QueryUserStatsResponse".into()
    }
}
/// Generated client implementations.
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Query defines the gRPC querier service.
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Queries the Params.
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryParamsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/dydxprotocol.stats.Query/Params",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("dydxprotocol.stats.Query", "Params"));
            self.inner.unary(req, path, codec).await
        }
        /// Queries StatsMetadata.
        pub async fn stats_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryStatsMetadataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryStatsMetadataResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/dydxprotocol.stats.Query/StatsMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("dydxprotocol.stats.Query", "StatsMetadata"));
            self.inner.unary(req, path, codec).await
        }
        /// Queries GlobalStats.
        pub async fn global_stats(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGlobalStatsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGlobalStatsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/dydxprotocol.stats.Query/GlobalStats",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("dydxprotocol.stats.Query", "GlobalStats"));
            self.inner.unary(req, path, codec).await
        }
        /// Queries UserStats.
        pub async fn user_stats(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryUserStatsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryUserStatsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/dydxprotocol.stats.Query/UserStats",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("dydxprotocol.stats.Query", "UserStats"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// MsgUpdateParams is the Msg/UpdateParams request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// The parameters to update. Each field must be set.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for MsgUpdateParams {
    const NAME: &'static str = "MsgUpdateParams";
    const PACKAGE: &'static str = "dydxprotocol.stats";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.stats.MsgUpdateParams".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.stats.MsgUpdateParams".into()
    }
}
/// MsgUpdateParamsResponse is the Msg/UpdateParams response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
impl ::prost::Name for MsgUpdateParamsResponse {
    const NAME: &'static str = "MsgUpdateParamsResponse";
    const PACKAGE: &'static str = "dydxprotocol.stats";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.stats.MsgUpdateParamsResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.stats.MsgUpdateParamsResponse".into()
    }
}
/// Generated client implementations.
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Msg defines the Msg service.
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MsgClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MsgClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MsgClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// UpdateParams updates the Params in state.
        pub async fn update_params(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateParams>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateParamsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/dydxprotocol.stats.Msg/UpdateParams",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("dydxprotocol.stats.Msg", "UpdateParams"));
            self.inner.unary(req, path, codec).await
        }
    }
}
