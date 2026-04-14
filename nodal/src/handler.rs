use crate::{RequestContext, ServiceContext};
use async_trait::async_trait;
use bytes::Bytes;
use std::fmt::Debug;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[async_trait]
pub trait EndpointHandler<Context>: Debug + Send + Sync
where
    Context: ServiceContext,
{
    async fn handle_request(
        &self,
        rqctx: RequestContext<Context>,
        body: Bytes,
    ) -> Result<Bytes, BoxError>;
}
