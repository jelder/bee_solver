use regex::Regex;
use std::collections::HashSet;

const DICT_ZSTD: &[u8] = include_bytes!("../dict.txt.zst");

pub fn get_dict() -> Vec<String> {
    let dict = zstd::decode_all(DICT_ZSTD).expect("Failed to decode dictionary");
    let dict = String::from_utf8_lossy(&dict);
    return dict.lines().map(|s| s.to_string()).collect();
}

pub struct Game {
    pub center: char,
    pub ring: [char; 6],
}

impl Game {
    pub fn new(center: char, ring: &String) -> Self {
        Game {
            center: center.to_ascii_lowercase(),
            ring: ring
                .to_ascii_lowercase()
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .unwrap(),
        }
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
        let mut plays: Vec<Play> = get_dict()
            .into_iter()
            .filter(|word| word.contains(self.center))
            .filter(|word| regex.is_match(word))
            .map(|word| Play(word.to_string()))
            .collect();

        plays.sort_by_key(|play| play.score());
        plays.reverse();
        plays
    }
}

pub struct Play(pub String);

impl Play {
    pub fn is_pangram(&self) -> bool {
        let mut seen = HashSet::new();
        for c in self.0.chars() {
            seen.insert(c);
        }
        seen.len() == 7
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
    fn it_works() {
        let game = Game {
            center: 'a',
            ring: ['h', 'n', 'p', 'd', 'o', 'e'],
        };
        let plays = game.plays();
        assert!(!plays.is_empty());
        assert_eq!(plays[0].0, "openhanded");
        assert_eq!(plays[0].is_pangram(), true);
    }
}
