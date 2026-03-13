use axum::{
    routing::get,
    Router,
    Json,
};
use rusqlite::{Connection, Result as SqlResult};
use serde::Serialize;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

#[derive(Serialize)]
struct Player {
    id: i32,
    name: String,
    tg_username: String,
    rank: String,
}

#[derive(Serialize)]
struct Achievement {
    id: i32,
    date: String,
    event: String,
    rating: i32,
    link: Option<String>,
}

struct AppState {
    db_path: String,
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    let state = Arc::new(AppState {
        db_path: "../../ITMO-team-site/ITMO_team/db.sqlite3".to_string(),
    });

    let app = Router::new()
        .route("/api/players", get(get_players))
        .route("/api/achievements", get(get_achievements))
        .layer(cors)
        .with_state(state);

    let addr = "127.0.0.1:3000";
    println!("Backend running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_players(axum::extract::State(state): axum::extract::State<Arc<AppState>>) -> Json<Vec<Player>> {
    let res = (|| -> SqlResult<Vec<Player>> {
        let conn = Connection::open(&state.db_path)?;
        let mut stmt = conn.prepare("SELECT id, name, tg_username, rank FROM main_player")?;
        let items = stmt.query_map([], |row| {
            Ok(Player {
                id: row.get(0)?,
                name: row.get(1)?,
                tg_username: row.get(2)?,
                rank: row.get(3)?,
            })
        })?;
        
        let mut players = Vec::new();
        for item in items {
            players.push(item?);
        }
        Ok(players)
    })();

    match res {
        Ok(players) => Json(players),
        Err(e) => {
            eprintln!("Error fetching players: {:?}", e);
            Json(vec![])
        }
    }
}

async fn get_achievements(axum::extract::State(state): axum::extract::State<Arc<AppState>>) -> Json<Vec<Achievement>> {
    let res = (|| -> SqlResult<Vec<Achievement>> {
        let conn = Connection::open(&state.db_path)?;
        let mut stmt = conn.prepare("SELECT id, date, event, rating, link FROM main_achievment")?;
        let items = stmt.query_map([], |row| {
            Ok(Achievement {
                id: row.get(0)?,
                date: row.get::<_, String>(1)?,
                event: row.get(2)?,
                rating: row.get(3)?,
                link: row.get(4).unwrap_or(None),
            })
        })?;
        
        let mut achievements = Vec::new();
        for item in items {
            achievements.push(item?);
        }
        Ok(achievements)
    })();

    match res {
        Ok(achievements) => Json(achievements),
        Err(e) => {
            eprintln!("Error fetching achievements: {:?}", e);
            Json(vec![])
        }
    }
}
