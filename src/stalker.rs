type HeroId = String;

#[derive(Debug)]
pub struct Player {
    pub mmr: usize,
    pub name: String,
    pub heroes: Vec<Hero>,
    pub total_games: u64,
}

#[derive(Debug)]
pub struct Hero {
    pub hero_id: u64,
    pub games: u64,
    pub win: u64,
    pub percent_rank: f64,
}

pub use std::collections::HashMap;

use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use serde::Deserialize;

use crate::error::KaarissouError;
pub struct Stalker {
    client: ClientWithMiddleware,
}

impl Stalker {
    pub fn new() -> Self {
        let client = ClientBuilder::new(reqwest::Client::new())
            .with(Cache(HttpCache {
                mode: CacheMode::Default,
                manager: CACacheManager::default(),
                options: None,
            }))
            .build();
        Self { client }
    }

    pub async fn reqwest(
        &self,
        team: String,
        steam_ids: impl Iterator<Item = u32>,
    ) -> Result<Vec<Player>, KaarissouError> {
        let mut players = Vec::new();

        for steam_id in steam_ids {
            println!("Info");
            let info = self
                .client
                .get(format!("https://api.opendota.com/api/players/{steam_id}"))
                .send()
                .await?
                .json::<HashMap<String, serde_json::Value>>()
                .await?;

            println!("WinLose");
            let wl = self
                .client
                .get(format!(
                    "https://api.opendota.com/api/players/{steam_id}/wl"
                ))
                .send()
                .await?
                .json::<HashMap<String, serde_json::Value>>()
                .await?;
            let total_games = wl["win"].as_u64().unwrap() + wl["lose"].as_u64().unwrap();
            println!("Ranking");
            let mut rankings = self
                .client
                .get(format!(
                    "https://api.opendota.com/api/players/{steam_id}/rankings"
                ))
                .send()
                .await?
                .json::<Vec<HashMap<String, serde_json::Value>>>()
                .await?;

            println!("Hero");
            let mut heroes_data = self
                .client
                .get(format!(
                    "https://api.opendota.com/api/players/{steam_id}/heroes"
                ))
                .send()
                .await?
                .json::<Vec<HashMap<String, serde_json::Value>>>()
                .await?;

            let name = info["profile"]["personaname"]
                .as_str()
                .unwrap_or("No name")
                .to_owned();
            let mmr = info["mmr_estimate"]["estimate"].as_u64().unwrap_or(0) as usize;

            let mut heroes = Vec::new();
            while !heroes_data.is_empty() || !rankings.is_empty() {
                let hero = heroes_data.swap_remove(0);
                let hero_id = hero["hero_id"]
                    .as_str()
                    .expect("id as str")
                    .parse::<u64>()
                    .expect("valid hero id");
                let percent_rank = rankings
                    .drain_filter(|r| r["hero_id"].as_u64().expect("id as number") == hero_id)
                    .next()
                    .and_then(|map| map["percent_rank"].as_f64())
                    .unwrap_or_default();

                heroes.push(Hero {
                    hero_id,
                    games: hero["games"].as_u64().expect("games as u64"),
                    win: hero["win"].as_u64().expect("win as u64"),
                    percent_rank,
                });
            }
            players.push(Player {
                name,
                mmr,
                heroes,
                total_games,
            });
        }

        Ok(players)
    }
}

#[tokio::test]
async fn stalk() {
    let s = Stalker::new();
    let e = s
        .reqwest("team".to_owned(), std::iter::once(1143095430))
        .await
        .unwrap();
    let p_name = &e[0].name;
    let p_mmr = &e[0].mmr;
    let p_heroes = &e[0].heroes[..2];
    let p_wl = &e[0].total_games;
    println!("{p_name} {p_mmr} {:?} {:?}", p_heroes, p_wl);
}
// https://api.opendota.com/api/players/{account_id}/rankings
// [

//     {
//         "hero_id": "string",
//         "score": 0,
//         "percent_rank": 0,
//         "card": 0
//     }

// ]
// https://api.opendota.com/api/players/{account_id}/heroes
// [

//     {
//         "hero_id": "string",
//         "last_played": 0,
//         "games": 0,
//         "win": 0,
//         "with_games": 0,
//         "with_win": 0,
//         "against_games": 0,
//         "against_win": 0
//     }
// ]
// https://api.opendota.com/api/players/{account_id}/wl
// {

//     "win": 0,
//     "lose": 0

// }
