use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
    ApplicationCommandAutocomplete = 4,
    ModalSubmit = 5,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    // Unique ID of the interaction
    pub id: String,

    // Type of the interaction
    #[serde(rename = "type")]
    pub kind: InteractionType,
    // TODO
}
