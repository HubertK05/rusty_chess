pub mod move_parser;

use std::collections::HashMap;
use anyhow::Context;
use reqwest::{Request, Method, Url, Client};
use serde::{Serialize, Deserialize, Deserializer, de::Visitor};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct OpeningBook(HashMap<String, (String, u32)>);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PositionDescription {
    pub moves: Vec<MoveDescription>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct MoveDescription {
    pub san: String,
    pub white: u32,
    pub draws: u32,
    pub black: u32,
}

pub async fn get_opening_book() -> Result<PositionDescription, OpeningBookError> {
    let client = Client::new();

    let res = client.get("https://explorer.lichess.ovh/masters?moves=3&topGames=0").send().await.context("Request failed")?;
    let desc = res.json::<PositionDescription>().await.context("Deserialization failed")?;

    Ok(desc)
}

#[derive(Error, Debug)]
pub enum OpeningBookError {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}
