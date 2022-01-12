extern crate basketball_rs;
extern crate reqwest;

#[cfg(test)]
extern crate tokio;

#[cfg(test)]
mod tests { 
    use basketball_rs::*;
    use tokio;
    use reqwest::Error;

    #[tokio::test]
    async fn test_team()-> Result<(), Error>{
        let team_url = r#"https://www.balldontlie.io/api/v1/teams/14"#;
        let team: Team = reqwest::get(team_url)
                .await?
                .json()
                .await?;
        assert_eq!(14, team.id());
        assert_eq!("LAL", team.abbreviation());
        assert_eq!("Los Angeles", team.city());
        assert_eq!("West", team.conference());
        assert_eq!("Pacific", team.division());
        assert_eq!("Los Angeles Lakers", team.full_name());
        assert_eq!("Lakers", team.name());
        Ok(())
    }
    #[tokio::test]
    async fn test_player() -> Result<(), Error>{
        let player_url = r#"https://www.balldontlie.io/api/v1/players/237"#;
        let player: Player = reqwest::get(player_url)
                .await?
                .json()
                .await?;
        let team_url = r#"https://www.balldontlie.io/api/v1/teams/14"#;
        let team: Team = reqwest::get(team_url)
                .await?
                .json()
                .await?;
        assert_eq!(237, player.id());
        assert_eq!("LeBron", player.first_name());
        assert_eq!("James", player.last_name());
        assert_eq!("F", player.position());
        assert_eq!(Some(6), player.height_feet());
        assert_eq!(Some(8), player.height_inches());
        assert_eq!(Some(250), player.weight_pounds());
        assert_eq!(&team, player.team());
        Ok(())
    }

    #[tokio::test]
    async fn test_game() -> Result<(), Error> {
        let game_url = r#"https://www.balldontlie.io/api/v1/games/1"#;
        let home_team_url = r#"https://www.balldontlie.io/api/v1/teams/2"#;
        let visitor_team_url = r#"https://www.balldontlie.io/api/v1/teams/23"#;
        let game: Game = reqwest::get(game_url)
                .await?
                .json()
                .await?;
        let home_team: Team = reqwest::get(home_team_url)
                .await?
                .json()
                .await?;
        let visitor_team: Team = reqwest::get(visitor_team_url)
                .await?
                .json()
                .await?;

        assert_eq!(1, game.id());
        assert_eq!("2018-10-16 00:00:00 UTC", game.date());
        assert_eq!(105, game.home_team_score());
        assert_eq!(87, game.visitor_team_score());
        assert_eq!(2018, game.season());
        assert_eq!(4, game.period());
        assert_eq!("Final", game.status());
        assert_eq!(" ", game.time());
        assert_eq!(false, game.postseason());
        assert_eq!(&home_team, game.home_team());
        assert_eq!(&visitor_team, game.visitor_team());
        Ok(())
    }




}