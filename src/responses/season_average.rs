use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SeasonAverage {
    pub games_played: u32,
    pub player_id: u32,
    pub season: u32,
    pub min: String,
    pub fgm: f32,
    pub fga: f32,
    pub fg3m: f32,
    pub fg3a: f32,
    pub ftm: f32,
    pub oreb: f32,
    pub dreb: f32,
    pub reb: f32,
    pub ast: f32,
    pub stl: f32,
    pub blk: f32,
    pub turnover: f32,
    pub pf: f32,
    pub pts: f32,
    pub fg_pct: f32,
    pub fg3_pct: f32,
    pub ft_pct: f32,
}

impl SeasonAverage {
    

    /// Get a reference to the season average's games played.
    pub fn games_played(&self) -> u32 {
        self.games_played
    }

    /// Get a reference to the season average's player id.
    pub fn player_id(&self) -> u32 {
        self.player_id
    }

    /// Get a reference to the season average's season.
    pub fn season(&self) -> u32 {
        self.season
    }

    /// Get a reference to the season average's min.
    pub fn min(&self) -> &str {
        self.min.as_ref()
    }

    /// Get a reference to the season average's fgm.
    pub fn fgm(&self) -> f32 {
        self.fgm
    }

    /// Get a reference to the season average's fga.
    pub fn fga(&self) -> f32 {
        self.fga
    }

    /// Get a reference to the season average's fg3m.
    pub fn fg3m(&self) -> f32 {
        self.fg3m
    }

    /// Get a reference to the season average's fg3a.
    pub fn fg3a(&self) -> f32 {
        self.fg3a
    }

    /// Get a reference to the season average's ftm.
    pub fn ftm(&self) -> f32 {
        self.ftm
    }

    /// Get a reference to the season average's oreb.
    pub fn oreb(&self) -> f32 {
        self.oreb
    }

    /// Get a reference to the season average's dreb.
    pub fn dreb(&self) -> f32 {
        self.dreb
    }

    /// Get a reference to the season average's reb.
    pub fn reb(&self) -> f32 {
        self.reb
    }

    /// Get a reference to the season average's ast.
    pub fn ast(&self) -> f32 {
        self.ast
    }

    /// Get a reference to the season average's stl.
    pub fn stl(&self) -> f32 {
        self.stl
    }

    /// Get a reference to the season average's blk.
    pub fn blk(&self) -> f32 {
        self.blk
    }

    /// Get a reference to the season average's turnover.
    pub fn turnover(&self) -> f32 {
        self.turnover
    }

    /// Get a reference to the season average's pf.
    pub fn pf(&self) -> f32 {
        self.pf
    }

    /// Get a reference to the season average's pts.
    pub fn pts(&self) -> f32 {
        self.pts
    }

    /// Get a reference to the season average's fg pct.
    pub fn fg_pct(&self) -> f32 {
        self.fg_pct
    }

    /// Get a reference to the season average's fg3 pct.
    pub fn fg3_pct(&self) -> f32 {
        self.fg3_pct
    }

    /// Get a reference to the season average's ft pct.
    pub fn ft_pct(&self) -> f32 {
        self.ft_pct
    }
}