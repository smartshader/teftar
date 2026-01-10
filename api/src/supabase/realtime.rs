// Realtime Module - Foundation for WebSocket-based real-time features
//
// NOTE: This is a foundational implementation. For full WebSocket support, you'll need:
// 1. Add to Cargo.toml: tokio-tungstenite = "0.21", futures = "0.3"
// 2. Implement WebSocket connection management
// 3. Implement Phoenix protocol message serialization
// 4. Add connection state management and reconnection logic
//
// This module provides the data structures and basic client setup.
// The actual WebSocket implementation should be added when needed.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Channel states following Phoenix protocol
#[derive(Debug, Clone, PartialEq)]
pub enum ChannelState {
    Closed,
    Joining,
    Joined,
    Leaving,
    Errored,
}

/// Event types supported by Supabase Realtime
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventType {
    Broadcast,
    Presence,
    PostgresChanges,
    System,
}

/// Phoenix protocol message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoenixMessage {
    pub join_ref: Option<String>,
    pub ref_id: Option<String>,
    pub topic: String,
    pub event: String,
    pub payload: serde_json::Value,
}

/// Channel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConfig {
    pub broadcast: BroadcastConfig,
    pub presence: PresenceConfig,
    pub private: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastConfig {
    pub self_send: bool,
    pub ack: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenceConfig {
    pub key: String,
}

impl Default for ChannelConfig {
    fn default() -> Self {
        Self {
            broadcast: BroadcastConfig {
                self_send: false,
                ack: false,
            },
            presence: PresenceConfig { key: String::new() },
            private: false,
        }
    }
}

/// Postgres change event types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PostgresChangeEvent {
    Insert,
    Update,
    Delete,
    #[serde(rename = "*")]
    All,
}

/// Postgres change payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostgresChangePayload {
    pub schema: String,
    pub table: String,
    pub commit_timestamp: String,
    #[serde(rename = "eventType")]
    pub event_type: String,
    pub new: HashMap<String, serde_json::Value>,
    pub old: HashMap<String, serde_json::Value>,
    pub errors: Option<Vec<String>>,
}

/// Postgres change filter
#[derive(Debug, Clone)]
pub struct PostgresChangeFilter {
    pub event: PostgresChangeEvent,
    pub schema: String,
    pub table: String,
    pub filter: Option<String>, // PostgREST filter syntax
}

/// Presence state entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenceEntry {
    pub presence_ref: String,
    #[serde(flatten)]
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Presence state (key -> presence entries)
pub type PresenceState = HashMap<String, Vec<PresenceEntry>>;

/// Broadcast message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastMessage {
    pub event: String,
    pub payload: serde_json::Value,
    #[serde(rename = "type")]
    pub msg_type: String, // "broadcast"
}

/// Realtime client configuration
#[derive(Debug, Clone)]
pub struct RealtimeConfig {
    pub endpoint: String,
    pub api_key: String,
    pub access_token: Option<String>,
    pub heartbeat_interval_ms: u64,
    pub timeout_ms: u64,
    pub reconnect_after_ms: Vec<u64>, // Backoff intervals
}

impl Default for RealtimeConfig {
    fn default() -> Self {
        Self {
            endpoint: String::new(),
            api_key: String::new(),
            access_token: None,
            heartbeat_interval_ms: 25000,
            timeout_ms: 10000,
            reconnect_after_ms: vec![1000, 2000, 5000, 10000],
        }
    }
}

/// Channel subscription status
#[derive(Debug, Clone, PartialEq)]
pub enum SubscriptionStatus {
    Subscribed,
    TimedOut,
    Closed,
    ChannelError,
}

// Placeholder for future WebSocket implementation
// When implementing, you'll need:
// 1. WebSocket connection management with tokio-tungstenite
// 2. Phoenix protocol message serialization/deserialization
// 3. Heartbeat mechanism using tokio::time::interval
// 4. Channel state tracking and automatic rejoin logic
// 5. Send buffer for queueing messages when disconnected
// 6. Presence diff tracking and state synchronization
// 7. Event callback registration and dispatch system

/// Realtime Client (placeholder for WebSocket implementation)
pub struct RealtimeClient {
    pub config: RealtimeConfig,
    // Future fields:
    // conn: Option<WebSocket>,
    // channels: HashMap<String, RealtimeChannel>,
    // ref_counter: AtomicU64,
    // send_buffer: Arc<Mutex<Vec<PhoenixMessage>>>,
}

impl RealtimeClient {
    pub fn new(endpoint: String, api_key: String) -> Self {
        Self {
            config: RealtimeConfig {
                endpoint,
                api_key,
                ..Default::default()
            },
        }
    }

    pub fn with_access_token(mut self, token: String) -> Self {
        self.config.access_token = Some(token);
        self
    }

    pub fn with_heartbeat_interval(mut self, interval_ms: u64) -> Self {
        self.config.heartbeat_interval_ms = interval_ms;
        self
    }

    // Future methods to implement:
    // pub async fn connect(&mut self) -> Result<(), RealtimeError>
    // pub async fn disconnect(&mut self) -> Result<(), RealtimeError>
    // pub fn channel(&mut self, topic: &str, config: ChannelConfig) -> &mut RealtimeChannel
    // pub async fn remove_channel(&mut self, topic: &str) -> Result<(), RealtimeError>
    // pub fn set_auth(&mut self, token: String)
}

/// Realtime Channel (placeholder)
pub struct RealtimeChannel {
    pub topic: String,
    pub config: ChannelConfig,
    pub state: ChannelState,
    // Future fields:
    // join_ref: String,
    // push_buffer: Vec<PhoenixMessage>,
    // presence_state: PresenceState,
    // event_handlers: HashMap<String, Vec<EventCallback>>,
}

impl RealtimeChannel {
    pub fn new(topic: String, config: ChannelConfig) -> Self {
        Self {
            topic,
            config,
            state: ChannelState::Closed,
        }
    }

    // Future methods to implement:
    // pub async fn subscribe<F>(&mut self, callback: F) -> Result<SubscriptionStatus, RealtimeError>
    //     where F: Fn(SubscriptionStatus) + Send + 'static
    // pub async fn unsubscribe(&mut self) -> Result<String, RealtimeError>
    // pub fn on<F>(&mut self, event_type: EventType, filter: EventFilter, callback: F)
    //     where F: Fn(serde_json::Value) + Send + 'static
    // pub async fn track(&mut self, metadata: HashMap<String, serde_json::Value>) -> Result<(), RealtimeError>
    // pub async fn untrack(&mut self) -> Result<(), RealtimeError>
    // pub fn presence_state(&self) -> &PresenceState
    // pub async fn send(&mut self, message: BroadcastMessage) -> Result<String, RealtimeError>
}

/// Error types for Realtime operations
#[derive(Debug)]
pub enum RealtimeError {
    ConnectionError(String),
    ChannelError(String),
    Timeout,
    AuthError(String),
    SerializationError(String),
}

impl std::fmt::Display for RealtimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RealtimeError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            RealtimeError::ChannelError(msg) => write!(f, "Channel error: {}", msg),
            RealtimeError::Timeout => write!(f, "Operation timed out"),
            RealtimeError::AuthError(msg) => write!(f, "Auth error: {}", msg),
            RealtimeError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for RealtimeError {}

// Helper functions for future implementation

/// Generate next message reference
pub fn next_ref(counter: &mut u64) -> String {
    *counter += 1;
    counter.to_string()
}

/// Calculate reconnection delay with exponential backoff
pub fn reconnect_delay(tries: usize, intervals: &[u64]) -> u64 {
    intervals
        .get(tries.saturating_sub(1))
        .copied()
        .unwrap_or(*intervals.last().unwrap_or(&10000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reconnect_delay() {
        let intervals = vec![1000, 2000, 5000, 10000];
        assert_eq!(reconnect_delay(1, &intervals), 1000);
        assert_eq!(reconnect_delay(2, &intervals), 2000);
        assert_eq!(reconnect_delay(3, &intervals), 5000);
        assert_eq!(reconnect_delay(4, &intervals), 10000);
        assert_eq!(reconnect_delay(10, &intervals), 10000); // Max backoff
    }

    #[test]
    fn test_next_ref() {
        let mut counter = 0;
        assert_eq!(next_ref(&mut counter), "1");
        assert_eq!(next_ref(&mut counter), "2");
        assert_eq!(next_ref(&mut counter), "3");
    }

    #[test]
    fn test_channel_config_default() {
        let config = ChannelConfig::default();
        assert_eq!(config.broadcast.self_send, false);
        assert_eq!(config.broadcast.ack, false);
        assert_eq!(config.private, false);
    }
}
