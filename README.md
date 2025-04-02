# NYT Spelling Bee Solver

A CLI and library for solving NYT's irritating but addictive daily word game [Spelling Bee](https://www.nytimes.com/puzzles/spelling-bee).

This uses a slightly larger dictionary than the real game, but the same scoring algorithm. You'll get some false positives. 

## Installation

```
cargo install bee_solver
```

### Web Mode

```
wasm-pack build --release --target web
```
