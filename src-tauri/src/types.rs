use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{ RwLock, mpsc};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct LinkedPath {
    pub name: String,
    pub path: PathBuf,
}

// Enum to represent the Network type
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")] // Matches TypeScript structure
pub struct Network {
        pub name: String,
        pub linked_paths: Vec<LinkedPath>,
}
#[derive(Serialize, Deserialize, Clone)]
pub enum ServerMode {
    LocalHost,
    Internet,
    DarkWeb,
}
pub type NetworkName = String;

pub type ShutdownServerMap = Arc<RwLock<HashMap<NetworkName, Vec<ServerGroup>>>>;


#[derive(Clone)]
pub struct ServerGroup {
    pub id: u64,
    pub addresses: Vec<Address>,
    pub tx: mpsc::Sender<()>,
}
#[derive(Clone,Serialize, Deserialize)]
pub struct ServerGroupSerde {
    pub id: u64,
    pub addresses: Vec<Address>,
}

#[derive(Clone,Serialize, Deserialize)]
pub struct Address {
    pub ip: String,
    pub port: u16,
}
#[derive(Clone)]
pub struct ServerIdState {
    pub server_id_counter: Arc<RwLock<u64>>,
}

impl ServerIdState {
    // Initialize the ServerState
    pub fn new() -> Self {
        ServerIdState {
            server_id_counter: Arc::new(RwLock::new(0)),
        }
    }

    // Increment and return the next available server ID
    pub async fn generate_server_id(&self) -> u64 {
        let mut counter = self.server_id_counter.write().await;
        *counter += 1;
        *counter
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Failed to receive the directory path.")]
    RecvError(#[from] tokio::sync::oneshot::error::RecvError),
}
// we must manually implement serde::Serialize
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FileError {
    #[error("failed to open file")]
    FileOpenError(#[from] std::io::Error),
    #[error("failed serialize json json")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("missing 'linked_paths' field in the JSON")]
    MissingLinkedPathsError,
}

impl serde::Serialize for FileError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FileWatcherError {
    #[error("Failed to create debouncer")]
    DebouncerCreationError(#[source] Box<dyn std::error::Error + Send>),
    #[error("Failed to send file change event")]
    SendError(#[source] Box<dyn std::error::Error + Send>),
    #[error("Failed to watch path")]
    WatchError(#[source] Box<dyn std::error::Error + Send>),
    #[error("Failed to receive file events")]
    RecvError(#[source] Box<dyn std::error::Error + Send>),
}

#[derive(Debug, thiserror::Error)]
pub enum MeasureLatencyError {
    #[error("Failed to execute command: {0}")]
    CommandError(#[from] std::io::Error),

    #[error("Failed to parse output: {0}")]
    ParseError(#[from] std::string::FromUtf8Error),

    #[error("Average latency not found in output")]
    AvgLatencyNotFound,

    #[error("Failed to get default gateway")]
    DefaultGatewayError,

    #[error("Error occurred: {0}")]
    Other(String),
}

#[derive(Debug, thiserror::Error)]
pub enum SendError {
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("I/O error: {0}")]
    IoError(String),
}
