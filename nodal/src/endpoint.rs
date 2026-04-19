use crate::{ServiceContext, ServiceState};
use async_trait::async_trait;
use bytes::Bytes;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::Arc;

/// Request wrapper type for endpoint request bodies
#[derive(Debug, Deserialize)]
pub struct Request<T: JsonSchema> {
    #[serde(flatten)]
    inner: T,
}

impl<T: JsonSchema> Request<T> {
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: JsonSchema> std::ops::Deref for Request<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// Successful response wrapper
#[derive(Debug, Serialize)]
pub struct Response<T>(pub T);

/// Request context.
#[non_exhaustive]
pub struct RequestContext<Context: ServiceContext> {
    pub(crate) nats: async_nats::Client,
    /// Service shared state.
    pub service: Arc<ServiceState<Context>>,
    /// Unique id for this request. Relies on the client generating this.
    pub request_id: String,
}

impl<Context: ServiceContext> RequestContext<Context> {
    /// Shared context.
    pub fn context(&self) -> &Context {
        &self.service.private
    }

    /// NATS connection.
    pub fn nats(&self) -> &async_nats::Client {
        &self.nats
    }
}

#[async_trait]
pub trait EndpointHandler<Context>: Debug + Send + Sync
where
    Context: ServiceContext,
{
    async fn handle_request(
        &self,
        rqctx: RequestContext<Context>,
        body: Bytes,
    ) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>>;
}
