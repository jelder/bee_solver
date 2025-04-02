use anyhow::Result;
use regex::Regex;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

static DICT: &str = include_str!("../dict.txt");

pub struct Game {
    pub center: char,
    pub ring: [char; 6],
}

impl Game {
    pub fn new(center: char, ring: &str) -> Result<Self> {
        let ring_chars: Vec<char> = ring.chars().collect();
        if ring_chars.len() != 6 {
            anyhow::bail!("Ring must contain exactly 6 characters");
        }
        let ring: [char; 6] = ring_chars
            .try_into()
            .map_err(|_| anyhow::anyhow!("Failed to convert ring to array of 6 characters"))?;

        Ok(Game {
            center: center.to_ascii_lowercase(),
            ring,
        })
    }

    fn to_regex(&self) -> Regex {
        Regex::new(&format!(
            "^[{center}{ring}]*$",
            center = self.center,
            ring = self.ring.iter().collect::<String>()
        ))
        .expect("Failed to create regex")
    }

    pub fn plays(&self) -> Vec<Play> {
        let regex = self.to_regex();
        let mut plays: Vec<Play> = DICT
            .lines()
            .filter(|word| word.contains(self.center))
            .filter(|word| regex.is_match(word))
            .map(|word| Play(word))
            .collect();

        plays.sort_by_key(|play| play.score());
        plays.reverse();
        plays
    }
}

pub struct Play(pub &'static str);

impl Play {
    pub fn is_pangram(&self) -> bool {
        let mut seen = HashSet::new();
        for c in self.0.chars() {
            seen.insert(c);
            if seen.len() == 7 {
                return true;
            }
        }
        return false;
    }

    pub fn score(&self) -> usize {
        let mut score = 0 as usize;
        if self.0.len() == 4 {
            score = score.saturating_add(1);
        } else {
            score = score.saturating_add(self.0.len());
        }
        if self.is_pangram() {
            score = score.saturating_add(7);
        }
        score
    }
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
        assert_eq!(plays[0].0, "openhanded");
        assert_eq!(plays[0].is_pangram(), true);
        assert_eq!(plays[0].score(), 17);
    }

    #[test]
    fn dict() {
        assert_eq!(DICT.lines().last().unwrap(), "zythum");
    }
}
