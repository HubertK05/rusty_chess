pub mod move_parser;

use std::{collections::HashMap, fs::File, io::Read, time::Duration};
use anyhow::Context;
use reqwest::{Client, StatusCode};
use serde::{Serialize, Deserialize};
use thiserror::Error;
use tokio::time::sleep;

use crate::board_setup::models::{FenNotation, Board};

use self::move_parser::{MoveParseError, parse_move};

const MIN_MOVE_POPULARITY: u32 = 2000;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpeningBook(pub HashMap<String, Vec<(String, u32)>>);

impl OpeningBook {
    pub fn from_file(path: &str) -> Self {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        serde_json::from_str::<OpeningBook>(&contents).unwrap()
    }
}

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

#[async_recursion::async_recursion]
pub async fn get_opening_book(book: &mut OpeningBook, fen: FenNotation) -> Result<(), OpeningBookError> {
    let client = Client::new();

    sleep(Duration::from_secs(1)).await;
    let mut res = client.get(&format!("https://explorer.lichess.ovh/masters?moves=10&topGames=0&fen={fen}")).send().await.context("Request failed")?;
    println!("{:?}", res);
    let mut time_limit = res.status() == StatusCode::TOO_MANY_REQUESTS;
    while time_limit {
        sleep(Duration::from_secs(61)).await;
        res = client.get(&format!("https://explorer.lichess.ovh/masters?moves=10&topGames=0&fen={fen}")).send().await.context("Request failed")?;
        println!("{:?}", res);
        time_limit = res.status() == StatusCode::TOO_MANY_REQUESTS;
    }
    let desc = res.json::<PositionDescription>().await.context("Deserialization failed")?;

    let draw_fen = fen.to_draw_fen();

    for played_move in desc.moves {
        let popularity = played_move.white + played_move.draws + played_move.black;
        if popularity >= MIN_MOVE_POPULARITY {
            println!("{}", played_move.san);
            book.0.entry(draw_fen.clone()).and_modify(|x| x.push((played_move.san.clone(), popularity))).or_insert(vec![(played_move.san.clone(), popularity)]);
            let played_move = parse_move(fen.clone(), played_move.san)?;
            let mut board = Board::try_from(fen.clone()).context("wrong fen")?;
            board.register_move(played_move).expect("failed to register move");
            let new_fen = FenNotation::try_from(&board).context("cannot create fen from board")?;
            get_opening_book(book, new_fen).await?;
        }
    };

    Ok(())
}

#[derive(Error, Debug)]
pub enum OpeningBookError {
    #[error("Move parse error")]
    MoveParseError(#[from] MoveParseError),
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}
