//! Header keys used in various NATS metadata.

/// Service RoFr version header.
pub const VERSION: &str = "RoFr-Version";
/// Service unique identifier header.
pub const SERVICE_UID: &str = "RoFr-Service-Uid";
/// Endpoint request id header.
pub const REQUEST_ID: &str = "RoFr-Request-Id";
/// Endpoint request id header.
pub const MESSAGE_ID: &str = "RoFr-Message-Id";
/// Endpoint request schema header.
pub const REQUEST_SCHEMA: &str = "RoFr-Request-Schema";
/// Endpoint response schema header.
pub const RESPONSE_SCHEMA: &str = "RoFr-Response-Schema";
/// Stream message schema header.
pub const MESSAGE_SCHEMA: &str = "RoFr-Message-Schema";
