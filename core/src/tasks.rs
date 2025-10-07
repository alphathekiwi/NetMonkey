use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    pub description: String,
    pub completed: bool,

    #[serde(skip)]
    #[allow(unused)]
    pub state: TaskState,
}

#[derive(Debug, Clone)]
pub enum TaskState {
    Idle,
    Editing,
}

impl Default for TaskState {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, Clone)]
pub enum TaskMessage {
    Completed(bool),
    Edit,
    DescriptionEdited(String),
    FinishEdition,
    Delete,
}
