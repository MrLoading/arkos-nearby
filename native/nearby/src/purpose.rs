use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::context::Context;
use crate::protocol::{ADDRESS, hex, mac};
use crate::{Result, require};

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SessionPurpose {
    #[default]
    Game,
    NetworkOnly,
}

#[derive(Clone)]
pub enum SessionIntent {
    Game(Box<Context>),
    NetworkOnly,
}

impl SessionIntent {
    pub fn context(&self) -> Option<&Context> {
        match self {
            Self::Game(context) => Some(context),
            Self::NetworkOnly => None,
        }
    }
    pub fn purpose(&self) -> SessionPurpose {
        match self {
            Self::Game(_) => SessionPurpose::Game,
            Self::NetworkOnly => SessionPurpose::NetworkOnly,
        }
    }
    pub fn key(&self) -> Value {
        self.context()
            .map(Context::key)
            .unwrap_or_else(|| json!({"purpose":"network_only"}))
    }
    pub fn revalidate(&self) -> Result<()> {
        if let Some(context) = self.context() {
            context.revalidate()?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkRoom {
    pub schema: u8,
    pub purpose: SessionPurpose,
    pub session_id: String,
    pub host_mac: String,
    pub host_ipv4: String,
}

impl NetworkRoom {
    pub fn new(id: String, host_mac: String) -> Result<Self> {
        let room = Self {
            schema: 1,
            purpose: SessionPurpose::NetworkOnly,
            session_id: id,
            host_mac,
            host_ipv4: ADDRESS.into(),
        };
        room.validate()?;
        Ok(room)
    }
    pub fn validate(&self) -> Result<()> {
        require(
            self.schema == 1
                && self.purpose == SessionPurpose::NetworkOnly
                && hex(&self.session_id, 32)
                && mac(&self.host_mac)
                && self.host_ipv4 == ADDRESS,
            "Invalid network room",
        )
    }
    pub fn observe(&self, bssid: &str, token: &str) -> Result<()> {
        self.validate()?;
        require(
            hex(token, 13) && self.session_id.starts_with(token) && self.host_mac == bssid,
            "Selected radio does not own this network room",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn network_intent_and_descriptor_cannot_supply_game_authority() {
        let intent = SessionIntent::NetworkOnly;
        assert!(intent.context().is_none());
        let room = NetworkRoom::new("a".repeat(32), "0c:c6:55:5d:8e:98".into()).unwrap();
        assert!(room.observe(&room.host_mac, &"a".repeat(13)).is_ok());
        assert!(room.observe(&room.host_mac, &"b".repeat(13)).is_err());
        assert!(crate::protocol::Room::decode(&serde_json::to_vec(&room).unwrap()).is_err());
        let mut value = serde_json::to_value(room).unwrap();
        value["core"] = json!({"id":"fake"});
        assert!(serde_json::from_value::<NetworkRoom>(value).is_err());
    }
}
