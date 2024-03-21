use serde::{Deserialize, Serialize};

use crate::SupermavenStateId;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateUpdateKind {
    StateUpdate,
}

// Outbound messages
#[derive(Debug, Serialize, Deserialize)]
pub struct StateUpdateMessage {
    // pub kind: "state_update",
    pub kind: StateUpdateKind,
    pub new_id: String,
    pub updates: Vec<StateUpdate>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StateUpdate {
    FileUpdate(FileUpdateMessage),
    CursorPositionUpdate(CursorPositionUpdateMessage),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileUpdateMessage {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CursorPositionUpdateMessage {
    pub path: String,
    pub offset: usize,
}

// Inbound messages coming in on stdout

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseItem {
    Text(String),
    Del(String),
    Dedent(String),
    End,
    Barrier,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupermavenResponse {
    pub state_id: SupermavenStateId,
    pub items: Vec<ResponseItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupermavenMetadataMessage {
    pub dust_strings: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupermavenTaskUpdateMessage {
    pub task: String,
    pub status: TaskStatus,
    pub percent_complete: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskStatus {
    InProgress,
    Complete,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupermavenActiveRepoMessage {
    pub repo_simple_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SupermavenPopupAction {
    OpenUrl { label: String, url: String },
    NoOp { label: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupermavenPopupMessage {
    pub message: String,
    pub actions: Vec<SupermavenPopupAction>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SupermavenMessage {
    Response(SupermavenResponse),
    Metadata(SupermavenMetadataMessage),
    Apology { message: Option<String> },
    ActivationRequest { activate_url: String },
    ActivationSuccess,
    Passthrough(Box<SupermavenMessage>),
    Popup(SupermavenPopupMessage),
    TaskStatus(SupermavenTaskUpdateMessage),
    ActiveRepo(SupermavenActiveRepoMessage),
}