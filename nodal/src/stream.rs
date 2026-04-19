use crate::{ServiceContext, ServiceState, header};
use async_nats::{
    HeaderMap, ToSubject,
    jetstream::context::{PublishAckFuture, PublishError},
    jetstream::message::PublishMessage,
};
use async_trait::async_trait;
use bytes::Bytes;
use serde::Serialize;
use std::{fmt::Debug, sync::Arc};

/// Stream context.
#[non_exhaustive]
pub struct StreamContext<Context: ServiceContext> {
    pub(crate) subject_prefix: String,
    pub(crate) jetstream: async_nats::jetstream::Context,
    /// Service shared state.
    pub service: Arc<ServiceState<Context>>,
}

impl<Context: ServiceContext> StreamContext<Context> {
    /// Shared context.
    pub fn context(&self) -> &Context {
        &self.service.private
    }

    /// NATS connection.
    pub fn nats(&self) -> async_nats::Client {
        self.jetstream.client()
    }

    /// NATS JetStream connection.
    pub fn jetstream(&self) -> &async_nats::jetstream::Context {
        &self.jetstream
    }

    /// Publish a message to the stream.
    pub async fn send<Subject: ToSubject>(
        &self,
        subject: Subject,
        message: &impl Serialize,
    ) -> Result<PublishAckFuture, PublishError> {
        self.jetstream
            .send_publish(
                format!("{}.{}", self.subject_prefix, subject.to_subject()),
                PublishMessage::build()
                    .payload(Bytes::from(serde_json::to_vec(message).unwrap()))
                    .header(header::MESSAGE_ID, ulid::Ulid::new().to_string()),
            )
            .await
    }

    /// Publish a message with headers to the stream.
    pub async fn send_with_headers<Subject: ToSubject>(
        &self,
        subject: Subject,
        headers: HeaderMap,
        message: &impl Serialize,
    ) -> Result<PublishAckFuture, PublishError> {
        self.jetstream
            .send_publish(
                format!("{}.{}", self.subject_prefix, subject.to_subject()),
                PublishMessage::build()
                    .payload(Bytes::from(serde_json::to_vec(message).unwrap()))
                    .headers(headers)
                    .header(header::MESSAGE_ID, ulid::Ulid::new().to_string()),
            )
            .await
    }
}

#[async_trait]
pub trait StreamHandler<Context>: Debug + Send + Sync
where
    Context: ServiceContext,
{
    async fn handle_stream(
        &self,
        rqctx: StreamContext<Context>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
