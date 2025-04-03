use gloo_utils::format::JsValueSerdeExt;
use itertools::Itertools;
use libflate::gzip::Decoder;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::io::Read;
use std::sync::OnceLock;
use thiserror::Error;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_plays(core: &str, ring: &str) -> Result<JsValue, GameError> {
    let core = match core.chars().exactly_one() {
        Ok(c) => c,
        Err(_) => return Err(GameError::InvalidCenterCharacter),
    };
    let game = match Game::new(core, ring) {
        Ok(game) => game,
        Err(e) => return Err(e),
    };
    let plays = game.plays();
    JsValue::from_serde(&plays).map_err(|_e| GameError::Unknown)
}

static COMPRESSED_DICT: &[u8] = include_bytes!("../dict.txt.gz");
static DICT: OnceLock<String> = OnceLock::new();

fn get_dict() -> &'static str {
    DICT.get_or_init(|| {
        let decoder = Decoder::new(COMPRESSED_DICT).expect("Failed to create gzip decoder");
        let mut decompressed = String::new();
        std::io::BufReader::new(decoder)
            .read_to_string(&mut decompressed)
            .expect("Failed to decompress dictionary");
        decompressed
    })
}

pub struct Game {
    pub center: char,
    pub ring: [char; 6],
}

#[derive(Error, Debug)]
#[wasm_bindgen]
pub enum GameError {
    #[error("Invalid center character")]
    InvalidCenterCharacter,
    #[error("Invalid ring length")]
    InvalidRingLength,
    #[error("Invalid ring characters")]
    InvalidRingCharacters,
    #[error("Unknown error")]
    Unknown,
}

impl Game {
    pub fn new(center: char, ring: &str) -> Result<Self, GameError> {
        let ring_chars: Vec<char> = ring.to_ascii_lowercase().chars().collect();
        if ring_chars.len() != 6 {
            return Err(GameError::InvalidRingLength);
        }
        let ring: [char; 6] = match ring_chars.try_into() {
            Ok(arr) => arr,
            Err(_) => return Err(GameError::InvalidRingCharacters),
        };

        Ok(Game {
            center: center.to_ascii_lowercase(),
            ring,
        })
    }

    fn to_regex(&self) -> Regex {
        Regex::new(&format!(
            "^[{center}{ring}]*$",
            center = self.center.to_ascii_lowercase(),
            ring = self.ring.iter().collect::<String>().to_ascii_lowercase()
        ))
        .expect("Failed to create regex")
    }

    pub fn plays(&self) -> Vec<Play> {
        let regex = self.to_regex();
        let mut plays: Vec<Play> = get_dict()
            .lines()
            .filter(|word| word.contains(self.center))
            .filter(|word| regex.is_match(word))
            .map(|word| Play::new(word))
            .collect();

        plays.sort_by_key(|play| play.score);
        plays.reverse();
        plays
    }
}

#[derive(Serialize, Deserialize)]
pub struct Play {
    pub word: &'static str,
    pub score: usize,
    pub is_pangram: bool,
}

impl Play {
    pub fn new(word: &'static str) -> Self {
        let is_pangram = is_pangram(word);
        let score = score(word, is_pangram);
        Play {
            word,
            score,
            is_pangram,
        }
    }
}

fn is_pangram(word: &str) -> bool {
    let mut seen = HashSet::new();
    for c in word.chars() {
        seen.insert(c);
        if seen.len() == 7 {
            return true;
        }
    }
    return false;
}

pub fn score(word: &str, is_pangram: bool) -> usize {
    let mut score = 0 as usize;
    if word.len() == 4 {
        score = score.saturating_add(1);
    } else {
        score = score.saturating_add(word.len());
    }
    if is_pangram {
        score = score.saturating_add(7);
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game() {
        let game = Game {
            center: 'a',
            ring: ['h', 'n', 'p', 'd', 'o', 'e'],
        };
        let plays = game.plays();
        assert!(!plays.is_empty());
        assert_eq!(plays[0].word, "openhanded");
        assert_eq!(plays[0].is_pangram, true);
        assert_eq!(plays[0].score, 17);
    }

    #[test]
    fn dict() {
        assert_eq!(get_dict().lines().last().unwrap(), "zythum");
    }
}
