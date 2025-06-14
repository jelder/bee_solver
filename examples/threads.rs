#![allow(unused)]
use rstrie::Trie;

const DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),   // Right
    (1, 0),   // Down
    (0, -1),  // Left
    (-1, 0),  // Up
    (1, 1),   // Down-Right
    (-1, -1), // Up-Left
    (1, -1),  // Down-Left
    (-1, 1),  // Up-Right
];

const GAME_WIDTH: usize = 6;
const GAME_HEIGHT: usize = 8;

type GRID<T> = [[T; GAME_WIDTH]; GAME_HEIGHT];
struct Game(GRID<char>);

impl Game {
    fn new(game_str: &str) -> Self {
        let mut grid: GRID<char> = [[' '; GAME_WIDTH]; GAME_HEIGHT];
        let cleaned_game_str: String = game_str.chars().filter(|c| !c.is_whitespace()).collect();
        dbg!(&cleaned_game_str);
        for (i, line) in cleaned_game_str
            .as_bytes()
            .chunks(GAME_WIDTH)
            .take(GAME_HEIGHT)
            .enumerate()
        {
            for (j, &c) in line.iter().enumerate() {
                grid[i][j] = (c as char).to_ascii_lowercase();
            }
        }
        assert_eq!(grid.len(), GAME_HEIGHT, "Grid height mismatch");
        for row in &grid {
            assert_eq!(row.len(), GAME_WIDTH, "Grid width mismatch");
        }
        Game(grid)
    }
}

#[derive(Debug, Clone)]
struct Coordinate {
    x: u8,
    y: u8,
}

#[derive(Debug, Clone)]
struct Play {
    word: String,
    path: Vec<Coordinate>, // Keeps the order of letters
    mask: u64,             // Bit mask representing the cells used
}

impl Play {
    fn print_ascii_art(&self, game: &Game) {
        let mut shape = vec![vec!['·'; GAME_WIDTH]; GAME_HEIGHT];

        for coord in &self.path {
            shape[coord.x as usize][coord.y as usize] = game.0[coord.x as usize][coord.y as usize];
        }

        for row in shape {
            println!("{}", row.iter().collect::<String>());
        }
        println!();
    }
}

fn find_plays(game: &Game, trie: &Trie<char, ()>) -> Vec<Play> {
    let mut visited = [[false; GAME_WIDTH]; GAME_HEIGHT];
    let mut found_words = Vec::new();

    for i in 0..GAME_HEIGHT {
        for j in 0..GAME_WIDTH {
            find_best_solution(
                game,
                trie,
                &mut visited,
                i,
                j,
                String::new(),
                Vec::new(),
                0,
                &mut found_words,
            );
        }
    }

    found_words
}

fn find_best_solution(
    game: &Game,
    trie: &Trie<char, ()>,
    visited: &mut [[bool; GAME_WIDTH]; GAME_HEIGHT],
    i: usize,
    j: usize,
    prefix: String,
    path: Vec<Coordinate>,
    mask: u64,
    plays: &mut Vec<Play>,
) {
    if visited[i][j] {
        return;
    }

    let mut new_prefix = prefix.clone();
    new_prefix.push(game.0[i][j]);

    let mut new_path = path.clone();
    new_path.push(Coordinate {
        x: i as u8,
        y: j as u8,
    });

    let bit_index = i * GAME_WIDTH + j;
    let new_path_mask = mask | (1 << bit_index);
    // println!(
    //     "Visiting cell {:?} ({}, {}), bit_index: {}, mask: {:b}",
    //     game.0[i][j], i, j, bit_index, new_path_mask
    // );

    // dbg!(&new_prefix, trie.is_prefix_str(&new_prefix));

    if !trie.is_prefix_str(&new_prefix) {
        return;
    }

    if trie.contains_key_str(&new_prefix) {
        plays.push(Play {
            word: new_prefix.clone(),
            path: new_path.clone(),
            mask: new_path_mask,
        });
    }

    visited[i][j] = true;

    for &(dx, dy) in &DIRECTIONS {
        let ni = i as isize + dx;
        let nj = j as isize + dy;

        if ni >= 0 && ni < GAME_HEIGHT as isize && nj >= 0 && nj < GAME_WIDTH as isize {
            find_best_solution(
                game,
                trie,
                visited,
                ni as usize,
                nj as usize,
                new_prefix.clone(),
                new_path.clone(),
                new_path_mask,
                plays,
            );
        }
    }

    visited[i][j] = false;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn test_find_plays() {
        let trie = build_trie();

        let game = Game::new(
            "
            THBSER
            ENAEPE
            TSNPTC
            IKOHRE
            LALSAC
            ZEPION
            GIOTSR
            OLEIDE
        ",
        );

        let plays = find_plays(&game, &trie);
        dbg!(&plays.get(0).map(|p| p.word.as_str()).unwrap());

        let expected_words: BTreeSet<_> = vec![
            "thank",
            "listen",
            "apologize",
            "respect",
            "share",
            "consider",
            "polite",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
        let found_words: BTreeSet<_> = plays.into_iter().map(|play| play.word).collect();
        let missing_words: BTreeSet<_> = expected_words.difference(&found_words).cloned().collect();

        assert_eq!(
            expected_words,
            expected_words.intersection(&found_words).cloned().collect(),
            "Did not find all expected words; missing: {:?}",
            missing_words
        );
    }

    #[test]
    fn test_game_new_grid_dimensions() {
        let game = Game::new(
            "
            ABCDEF
            FGHIJK
            KLMNOP
            PQRSTU
            UVWXYZ
            ZABCDE
            EFGHIJ
            JKLMNO
        ",
        );

        assert_eq!(game.0.len(), GAME_HEIGHT, "Grid height is incorrect");
        for row in &game.0 {
            assert_eq!(row.len(), GAME_WIDTH, "Grid width is incorrect");
        }
    }

    #[test]
    fn test_game_new_content() {
        let game = Game::new(
            "
            ABCDEF
            FGHIJK
            KLMNOP
            PQRSTU
            UVWXYZ
            ZABCDE
            EFGHIJ
            JKLMNO
        ",
        );

        dbg!(&game.0);
        assert_eq!(game.0[0][0], 'a', "First cell should be 'a'");
        assert_eq!(game.0[7][4], 'n', "Last cell should be 'n'");
        assert_eq!(game.0[3][2], 'r', "Middle cell should be 'r'");
    }

    #[test]
    fn test_find_plays_no_words() {
        let trie = Trie::new();
        let game = Game::new(
            "
            ABCDEF
            FGHIJK
            KLMNOP
            PQRSTU
            UVWXYZ
            ZABCDE
            EFGHIJ
            JKLMNO
        ",
        );

        let plays = find_plays(&game, &trie);
        assert!(plays.is_empty(), "No words should be found");
    }
}

fn build_trie() -> Trie<char, ()> {
    let mut trie: Trie<char, ()> = Trie::new();

    let dictionary = include_str!("/usr/share/dict/words");

    for word in dictionary.lines() {
        let word = word.trim().to_ascii_lowercase();
        if word.len() > 3 {
            trie.insert(word.chars(), ());
        }
    }

    trie
}

fn main() {
    let trie = build_trie();

    let game = Game::new(
        "
        THBSER
        ENAEPE
        TSNPTC
        IKOHRE
        LALSAC
        ZEPION
        GIOTSR
        OLEIDE
    ",
    );

    let plays = find_plays(&game, &trie);
    dbg!(&plays.len());
    for play in plays {
        println!("Found word: {}", play.word);
        play.print_ascii_art(&game);
    }
}
