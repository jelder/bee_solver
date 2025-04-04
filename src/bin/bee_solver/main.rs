use ansi_term::Style;
use bee_solver::Game;
use clap::Parser;

#[derive(Parser)]
/// Spelling Bee Solver
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// the center letter
    center: char,

    /// the ring letters
    #[clap(value_parser = validate_ring)]
    ring: String,
}

fn validate_ring(ring: &str) -> Result<String, String> {
    if ring.len() == 6 && ring.chars().all(|c| c.is_alphabetic()) {
        Ok(String::from(ring))
    } else {
        Err("Ring must contain exactly 6 letters".into())
    }
}

pub fn main() {
    let cli = Cli::parse();
    let game = match Game::new(cli.center, &cli.ring) {
        Ok(game) => game,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    let bold = Style::new().bold();

    let mut plays_by_score = std::collections::BTreeMap::new();
    for play in game.plays() {
        plays_by_score
            .entry(play.score)
            .or_insert_with(Vec::new)
            .push(play);
    }
    for (score, plays) in plays_by_score.iter().rev() {
        let mut play_str = String::new();
        for play in plays {
            play_str.push_str(&format!(
                "{} ",
                if play.is_pangram {
                    bold.paint(play.word)
                } else {
                    Style::default().paint(play.word)
                }
            ));
        }
        println!("{score:4} {play_str}");
    }
}
