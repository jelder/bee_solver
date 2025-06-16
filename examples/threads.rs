#![allow(unused)]
use rayon::prelude::*;
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
const TOTAL_CELLS: u64 = (GAME_WIDTH * GAME_HEIGHT) as u64;
const MIN_WORD_LENGTH: usize = 4; // Minimum word length to consider

type Grid<T> = [[T; GAME_WIDTH]; GAME_HEIGHT];
struct Game(Grid<char>);

impl Game {
    fn new(game_str: &str) -> Self {
        let mut grid: Grid<char> = [[' '; GAME_WIDTH]; GAME_HEIGHT];
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

impl Coordinate {
    /// Returns a u64 with a single bit set representing the coordinate's position on the board.
    fn mask(&self) -> u64 {
        let bit_index = self.x as usize * GAME_WIDTH + self.y as usize;
        1 << bit_index
    }
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
    let mut visited: Grid<bool> = [[false; GAME_WIDTH]; GAME_HEIGHT];
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
    found_words.sort_by(|a, b| b.word.len().cmp(&a.word.len()));
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
    let coord = Coordinate {
        x: i as u8,
        y: j as u8,
    };
    new_path.push(coord);

    if !trie.is_prefix_str(&new_prefix) {
        return;
    }

    let new_path_mask = new_path.iter().fold(0, |acc, coord| acc | coord.mask());

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

fn find_largest_compatible_group(plays: &[Play], preselected: &[Play]) -> Vec<Play> {
    // Calculate the mask for the preselected plays
    let preselected_mask: u64 = preselected.iter().fold(0, |acc, play| acc | play.mask);

    // Ensure preselected plays are compatible with each other
    assert!(
        preselected
            .iter()
            .all(|play| preselected_mask & play.mask == play.mask),
        "Preselected plays are not compatible with each other"
    );

    // Split the plays into chunks and process them in parallel
    let largest_group = plays
        .par_iter()
        .enumerate()
        .map(|(index, _)| {
            let mut largest_group = preselected.to_vec();

            fn backtrack(
                plays: &[Play],
                index: usize,
                current_group: &mut Vec<Play>,
                current_mask: u64,
                largest_group: &mut Vec<Play>,
            ) {
                if index == plays.len() {
                    // Update the largest group if the current group is larger
                    if current_group.len() > largest_group.len() {
                        *largest_group = current_group.clone();
                        eprintln!(
                            "New largest group found with {} plays: {:?}",
                            largest_group.len(),
                            largest_group.iter().map(|p| &p.word).collect::<Vec<_>>()
                        );
                    }
                    return;
                }

                let play = &plays[index];

                // Option 1: Include the current play if compatible
                if current_mask & play.mask == 0 {
                    current_group.push(play.clone());
                    backtrack(
                        plays,
                        index + 1,
                        current_group,
                        current_mask | play.mask,
                        largest_group,
                    );
                    current_group.pop();
                }

                // Option 2: Skip the current play
                backtrack(plays, index + 1, current_group, current_mask, largest_group);
            }

            backtrack(
                plays,
                index,
                &mut largest_group.clone(),
                preselected_mask,
                &mut largest_group,
            );
            largest_group
        })
        .max_by_key(|group| group.len()) // Find the largest group across all threads
        .unwrap_or_default();

    largest_group
}

fn find_all_compatible_groups(plays: &[Play], preselected: &[Play]) -> Vec<Vec<Play>> {
    let preselected_mask: u64 = preselected.iter().fold(0, |acc, play| acc | play.mask);

    assert!(
        preselected
            .iter()
            .all(|play| preselected_mask & play.mask == play.mask),
        "Preselected plays are not compatible with each other"
    );

    let mut all_groups = Vec::new();

    fn backtrack(
        plays: &[Play],
        index: usize,
        current_group: &mut Vec<Play>,
        current_mask: u64,
        all_groups: &mut Vec<Vec<Play>>,
    ) {
        if index == plays.len() {
            let leftovers = TOTAL_CELLS - current_mask.count_ones() as u64;
            if leftovers < MIN_WORD_LENGTH as u64 {
                all_groups.push(current_group.clone());
            }
            return;
        }

        let play = &plays[index];

        // Option 1: Include the current play if compatible
        if current_mask & play.mask == 0 {
            current_group.push(play.clone());
            backtrack(
                plays,
                index + 1,
                current_group,
                current_mask | play.mask,
                all_groups,
            );
            current_group.pop();
        }

        // Option 2: Skip the current play
        backtrack(plays, index + 1, current_group, current_mask, all_groups);
    }

    backtrack(
        plays,
        0,
        &mut preselected.to_vec(),
        preselected_mask,
        &mut all_groups,
    );

    all_groups
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

    #[test]
    fn test_coordinate_mask() {
        let coord = Coordinate { x: 2, y: 3 };
        let mask = coord.mask();
        assert_eq!(
            mask.count_ones(),
            1,
            "Mask should have exactly one bit set: {:064b}",
            mask,
        );
    }
}

fn build_trie() -> Trie<char, ()> {
    let mut trie: Trie<char, ()> = Trie::new();

    let dictionary = include_str!("/usr/share/dict/words");

    for word in dictionary.lines() {
        let word = word.trim().to_ascii_lowercase();
        if word.len() > MIN_WORD_LENGTH {
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

    let hints: Vec<Play> = vec![];
    let hints = plays
        .iter()
        .filter(|p| {
            false
            // || p.word == "polite"
            // || p.word == "listen"
            // || p.word == "respect"
            // || p.word == "thank"
            // || p.word == "share"
            // || p.word == "apologize"
            // || p.word == "consider"
        })
        .cloned()
        .collect::<Vec<Play>>();

    // dbg!(&hints.len());
    let all_groups = find_all_compatible_groups(&plays, &hints);

    println!("All valid groups of plays ({} groups):", all_groups.len());
    for (i, group) in all_groups.iter().enumerate() {
        // Calculate grid coverage
        let total_cells = (GAME_WIDTH * GAME_HEIGHT) as u64;
        let group_mask: u64 = group.iter().fold(0, |acc, play| acc | play.mask);
        let covered_cells = group_mask.count_ones() as u64;
        let coverage_percentage = (covered_cells as f64 / total_cells as f64) * 100.0;
        let leftovers = total_cells - covered_cells;

        println!(
            "Group {} ({} plays, {leftovers} leftovers {coverage_percentage:.2}% grid coverage):",
            i + 1,
            group.len(),
        );
        for play in group {
            println!("Word: {}", play.word);
            // play.print_ascii_art(&game);
        }
        println!("---");
    }
}
