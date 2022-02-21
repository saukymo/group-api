use tide::prelude::*;
use sqlx::{query_as, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub game_id: i32,
    pub name: String,
    pub author: Option<String>,
    pub publisher: Option<String>,
    pub description: String,
    pub quota: i32,
    pub cover: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewGame {
    pub name: String,
    pub author: Option<String>,
    pub publisher: Option<String>,
    pub description: String,
    pub quota: i32,
    pub cover: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameID {
    pub game_id: i32
}

impl Game {
    pub async fn add_new_game(new_game: NewGame, pg_conn: &PgPool) -> tide::Result<Game> {
        let created_game = query_as!(Game, r#"
        INSERT INTO games (
            name,
            author,
            publisher,
            description,
            quota,
            cover
        ) VALUES ($1, $2, $3, $4, $5, $6) RETURNING 
            game_id,
            name,
            author,
            publisher,
            description,
            quota,
            cover
        "#, new_game.name, new_game.author, new_game.publisher, new_game.description, new_game.quota, new_game.cover)
        .fetch_one(pg_conn).await?;

        Ok(created_game)
    }

    pub async fn get_games(pg_conn: &PgPool) -> tide::Result<Vec<Game>> {
        let games = query_as!(Game, r#"SELECT game_id, name, author, publisher, description, quota, cover FROM games"#).fetch_all(pg_conn).await?;

        Ok(games)
    }

    pub async fn get_game_by_game_id(game_id: i32, pg_conn: &PgPool) -> tide::Result<Option<Game>> {
        let game = query_as!(Game, r#"SELECT 
            game_id,
            name,
            author,
            publisher,
            description,
            quota,
            cover
            FROM games WHERE game_id=$1"#, game_id).fetch_optional(pg_conn).await?;

            Ok(game)
    }
}
