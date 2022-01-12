use super::Team;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    pub id: Option<i32>,
    pub ast: Option<u32>,
    pub blk: Option<u32>,
    pub dreb: Option<u32>,
    pub fg3_pct: Option<f32>,
    pub fg3a: Option<u32>,
    pub fg3m: Option<u32>,
    pub fg_pct: Option<f32>,
    pub fga: Option<u32>,
    pub fgm: Option<u32>,
    pub ft_pct: Option<f32>,
    pub fta: Option<u32>,
    pub ftm: Option<u32>,
    pub game: Option<ModGame>,
    pub min: Option<String>,
    pub oreb: Option<u32>,
    pub pf: Option<u32>,
    pub player: ModPlayer,
    pub pts: Option<u32>,
    pub reb: Option<u32>,
    pub stl: Option<u32>,
    pub team: Option<Team>,
    pub turnover: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ModPlayer {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub position: String,
    pub team_id: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ModGame {
    pub id: u32,
    pub date: String,
    pub home_team_id: u32,
    pub home_team_score: u32,
    pub season: u32,
    pub visitor_team_id: u32,
    pub visitor_team_score: u32,
}

impl Stats {
    /// Get a reference to the stats's id.
    pub fn id(&self) -> Option<i32> {
        self.id
    }

    /// Get a reference to the stats's ast.
    pub fn ast(&self) -> Option<u32> {
        self.ast
    }

    /// Get a reference to the stats's blk.
    pub fn blk(&self) -> Option<u32> {
        self.blk
    }

    /// Get a reference to the stats's dreb.
    pub fn dreb(&self) -> Option<u32> {
        self.dreb
    }

    /// Get a reference to the stats's fg3 pct.
    pub fn fg3_pct(&self) -> Option<f32> {
        self.fg3_pct
    }

    /// Get a reference to the stats's fg3a.
    pub fn fg3a(&self) -> Option<u32> {
        self.fg3a
    }

    /// Get a reference to the stats's fg3m.
    pub fn fg3m(&self) -> Option<u32> {
        self.fg3m
    }

    /// Get a reference to the stats's fg pct.
    pub fn fg_pct(&self) -> Option<f32> {
        self.fg_pct
    }

    /// Get a reference to the stats's fga.
    pub fn fga(&self) -> Option<u32> {
        self.fga
    }

    /// Get a reference to the stats's fgm.
    pub fn fgm(&self) -> Option<u32> {
        self.fgm
    }

    /// Get a reference to the stats's ft pct.
    pub fn ft_pct(&self) -> Option<f32> {
        self.ft_pct
    }

    /// Get a reference to the stats's fta.
    pub fn fta(&self) -> Option<u32> {
        self.fta
    }

    /// Get a reference to the stats's ftm.
    pub fn ftm(&self) -> Option<u32> {
        self.ftm
    }

    /// Get a reference to the stats's game.
    pub fn game(&self) -> &Option<ModGame> {
        &self.game
    }

    /// Get a reference to the stats's min.
    pub fn min(&self) -> Option<&String> {
        self.min.as_ref()
    }

    /// Get a reference to the stats's oreb.
    pub fn oreb(&self) -> Option<u32> {
        self.oreb
    }

    /// Get a reference to the stats's pf.
    pub fn pf(&self) -> Option<u32> {
        self.pf
    }

    /// Get a reference to the stats's player.
    pub fn player(&self) -> Option<&ModPlayer> {
        Some(&self.player)
    }

    /// Get a reference to the stats's pts.
    pub fn pts(&self) -> Option<u32> {
        self.pts
    }

    /// Get a reference to the stats's reb.
    pub fn reb(&self) -> Option<u32> {
        self.reb
    }

    /// Get a reference to the stats's stl.
    pub fn stl(&self) -> Option<u32> {
        self.stl
    }

    /// Get a reference to the stats's team.
    pub fn team(&self) -> &Option<Team> {
        &self.team
    }

    /// Get a reference to the stats's turnover.
    pub fn turnover(&self) -> Option<u32> {
        self.turnover
    }
}
