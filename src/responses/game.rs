use super::Team;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Game {
    pub id: u32,
    pub date: String,
    pub home_team_score: u32,
    pub visitor_team_score: u32,
    pub season: u32,
    pub period: u32,
    pub status: String,
    pub time: String,
    pub postseason: bool,
    pub home_team: Team,
    pub visitor_team: Team,
}


impl Game {
    /// Get a reference to the game's id.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Get a reference to the game's date.
    pub fn date(&self) -> &str {
        self.date.as_ref()
    }

    /// Get a reference to the game's home team score.
    pub fn home_team_score(&self) -> u32 {
        self.home_team_score
    }

    /// Get a reference to the game's visitor team score.
    pub fn visitor_team_score(&self) -> u32 {
        self.visitor_team_score
    }

    /// Get a reference to the game's season.
    pub fn season(&self) -> u32 {
        self.season
    }

    /// Get a reference to the game's status.
    pub fn status(&self) -> &str {
        self.status.as_ref()
    }

    /// Get a reference to the game's time.
    pub fn time(&self) -> &str {
        self.time.as_ref()
    }

    /// Get a reference to the game's postseason.
    pub fn postseason(&self) -> bool {
        self.postseason
    }

    /// Get a reference to the game's home team.
    pub fn home_team(&self) -> &Team {
        &self.home_team
    }

    /// Get a reference to the game's visitor team.
    pub fn visitor_team(&self) -> &Team {
        &self.visitor_team
    }

    /// Get a reference to the game's period.
    pub fn period(&self) -> u32 {
        self.period
    }
}
