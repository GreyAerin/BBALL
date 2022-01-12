use super::Team;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Player {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub position: String,
    pub height_feet: Option<u8>,
    pub height_inches: Option<u8>,
    pub weight_pounds: Option<u32>,
    pub team: Team,
}

impl Player {
    /// Get a reference to the player's id.
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Get a reference to the player's first name.
    pub fn first_name(&self) -> &str {
        self.first_name.as_ref()
    }

    /// Get a reference to the player's last name.
    pub fn last_name(&self) -> &str {
        self.last_name.as_ref()
    }

    /// Get a reference to the player's position.
    pub fn position(&self) -> &str {
        self.position.as_ref()
    }

    /// Get a reference to the player's height feet.
    pub fn height_feet(&self) -> Option<u8> {
        self.height_feet
    }

    /// Get a reference to the player's height inches.
    pub fn height_inches(&self) -> Option<u8> {
        self.height_inches
    }

    /// Get a reference to the player's weight pounds.
    pub fn weight_pounds(&self) -> Option<u32> {
        self.weight_pounds
    }

    /// Get a reference to the player's team.
    pub fn team(&self) -> &Team {
        &self.team
    }
}
