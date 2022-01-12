use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Team {
    pub id: u64,
    pub abbreviation: String,
    pub city: String,
    pub conference: String,
    pub division: String,
    pub full_name: String,
    pub name: String,
}

impl Team {
    /// Get a reference to the team's id.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Get a reference to the team's abbreviation.
    pub fn abbreviation(&self) -> &str {
        self.abbreviation.as_ref()
    }

    /// Get a reference to the team's city
    pub fn city(&self) -> &str {
        self.city.as_ref()
    }

    /// Get a reference to the team's conference.
    pub fn conference(&self) -> &str {
        self.conference.as_ref()
    }

    /// Get a reference to the team's division.
    pub fn division(&self) -> &str {
        self.division.as_ref()
    }

    /// Get a reference to the team's full name.
    pub fn full_name(&self) -> &str {
        self.full_name.as_ref()
    }

    /// Get a reference to the team's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}
