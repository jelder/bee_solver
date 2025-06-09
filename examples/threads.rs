use rstrie::Trie;

fn build_trie() -> Trie<char, ()> {
    let mut trie: Trie<char, ()> = Trie::new();

    // Include the dictionary file at compile time
    let dictionary = include_str!("/usr/share/dict/words");

    for word in dictionary.lines() {
        let word = word.trim().to_ascii_lowercase();
        if word.len() > 4 {
            trie.insert(word.chars(), ());
        }
    }

    trie
}

fn find_best_solution(
    game: &Vec<Vec<char>>,
    trie: &Trie<char, ()>,
    directions: &[(isize, isize)],
    visited: &mut Vec<Vec<bool>>,
    i: usize,
    j: usize,
    prefix: String,
    path: Vec<(usize, usize)>, // Track coordinates of each letter
    found_words: &mut Vec<(String, Vec<(usize, usize)>)>, // Include word and its path
) {
    if visited[i][j] {
        return;
    }

    let mut new_prefix = prefix.clone();
    new_prefix.push(game[i][j]);

    let mut new_path = path.clone();
    new_path.push((i, j)); // Add current coordinates to the path

    if !trie.is_prefix_str(&new_prefix) {
        return;
    }

    // Add to found_words if the prefix is a complete word
    if trie.contains_key_str(&new_prefix) {
        found_words.push((new_prefix.clone(), new_path.clone())); // Store word and its path
    }

    visited[i][j] = true;

    for &(dx, dy) in directions {
        let ni = i as isize + dx;
        let nj = j as isize + dy;

        if ni >= 0 && ni < game.len() as isize && nj >= 0 && nj < game[0].len() as isize {
            find_best_solution(
                game,
                trie,
                directions,
                visited,
                ni as usize,
                nj as usize,
                new_prefix.clone(),
                new_path.clone(),
                found_words,
            );
        }
    }

    visited[i][j] = false;
}

fn print_word_shape(game: &Vec<Vec<char>>, word: &str, path: &Vec<(usize, usize)>) {
    let mut shape = vec![vec!['·'; game[0].len()]; game.len()];

    for &(x, y) in path {
        shape[x][y] = game[x][y];
    }

    println!("Word: {}", word);
    for row in shape {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
}

fn main() {
    let trie = build_trie();

    let game_str = "
        ITSMES
        GNSUTS
        GIHCOG
        NSOANN
        IEWDCI
        CRTIGH
        AOCMIT
        STSELS
    ";

    let game: Vec<Vec<char>> = game_str
        .to_ascii_lowercase()
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().chars().collect())
        .collect();

    let directions = [
        (0, 1),   // Right
        (1, 0),   // Down
        (0, -1),  // Left
        (-1, 0),  // Up
        (1, 1),   // Down-Right
        (-1, -1), // Up-Left
        (1, -1),  // Down-Left
        (-1, 1),  // Up-Right
    ];

    let mut visited = vec![vec![false; game[0].len()]; game.len()];
    let mut found_words = Vec::new();

    for i in 0..game.len() {
        for j in 0..game[0].len() {
            find_best_solution(
                &game,
                &trie,
                &directions,
                &mut visited,
                i,
                j,
                String::new(),
                Vec::new(),
                &mut found_words,
            );
        }
    }

    // Remove duplicates and sort by length
    let unique_words: std::collections::HashSet<(String, Vec<(usize, usize)>)> =
        found_words.into_iter().collect();
    let mut found_words: Vec<(String, Vec<(usize, usize)>)> = unique_words.into_iter().collect();
    found_words.sort_by(|a, b| a.0.len().cmp(&b.0.len()));

    println!("Solution with the most words:");
    for (word, path) in found_words {
        print_word_shape(&game, &word, &path);
    }
}
