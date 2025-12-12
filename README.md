<h1 align="center">🎄 Advent of Code 2025 🦀</h1>


<p align="center">
  <img src="https://img.shields.io/badge/language-Rust-red.svg" alt="MIT License Badge"/>
  <img src="https://img.shields.io/badge/year-2025-orange.svg" alt="MIT License Badge"/>
  <a href="https://github.com/lagmoellertim/aoc-2025/blob/master/LICENSE" target="_blank"><img src="https://img.shields.io/badge/license-MIT-yellow.svg" alt="MIT License Badge"/></a>
</p>

---

## 🎄 Introduction

Welcome to my solutions for [Advent of Code 2025](https://adventofcode.com/2025)! This year, I am tackling the puzzles
using **Rust** with a custom CLI that supports benchmarking and interactive prompts.

## 🦌 Usage

You can run the solutions either by passing arguments directly or by using the interactive mode.

### Interactive Mode

If you run the binary without arguments (or with missing required arguments), the CLI will prompt you for the necessary
information.

```
$ cargo run
✔ Day · 1
✔ Part · 1
✔ Puzzle Input · input.txt
```

### Command Line Arguments

For quick execution or automation, you can pass arguments directly.

```
# Run Day 1, Part 1
cargo run -- -d 1 -p 1

# Run Day 1, Part 2 with a specific example input file
cargo run -- -d 1 -p 2 -i puzzle_inputs/day01/example.txt

# Benchmark Day 1, Part 1 with 1000 iterations
cargo run --release -- -d 1 -p 1 -r 1000
```

### Options

```
Usage: aoc-2025 [OPTIONS]

Options:
  -d, --day <DAY>      
  -p, --part <PART>    
  -i, --input <INPUT>  
  -r, --runs <RUNS>    [default: 1]
  -h, --help           Print help
  -V, --version        Print version
```

## 🎁 Progress

| Day | Puzzle Name         | Part 1 | Part 2 |                                      Code                                      |                   Puzzle                    |
|:---:|:--------------------|:------:|:------:|:------------------------------------------------------------------------------:|:-------------------------------------------:|
| 01  | Secret Entrance     |   ✅    |   ✅    | [Source](https://github.com/lagmoellertim/aoc-2025/tree/main/src/solver/day01) | [AoC](https://adventofcode.com/2025/day/1)  |
| 02  | Gift Shop           |   ✅    |   ✅    | [Source](https://github.com/lagmoellertim/aoc-2025/tree/main/src/solver/day02) | [AoC](https://adventofcode.com/2025/day/2)  |
| 03  | Lobby               |   ✅    |   ✅    | [Source](https://github.com/lagmoellertim/aoc-2025/tree/main/src/solver/day03) | [AoC](https://adventofcode.com/2025/day/3)  |
| 04  | Printing Department |   ✅    |   ✅    | [Source](https://github.com/lagmoellertim/aoc-2025/tree/main/src/solver/day04) | [AoC](https://adventofcode.com/2025/day/4)  |
| 05  | Cafeteria           |   ✅    |   ✅    | [Source](https://github.com/lagmoellertim/aoc-2025/tree/main/src/solver/day05) | [AoC](https://adventofcode.com/2025/day/5)  |
| 06  | Trash Compactor     |   ✅    |   ✅    | [Source](https://github.com/lagmoellertim/aoc-2025/tree/main/src/solver/day06) | [AoC](https://adventofcode.com/2025/day/6)  |
| 07  | Laboratories        |   ✅    |   ✅    | [Source](https://github.com/lagmoellertim/aoc-2025/tree/main/src/solver/day07) | [AoC](https://adventofcode.com/2025/day/7)  |
| 08  | Playground          |   ✅    |   ✅    | [Source](https://github.com/lagmoellertim/aoc-2025/tree/main/src/solver/day08) | [AoC](https://adventofcode.com/2025/day/8)  |
| 09  | Movie Theater       |   ✅    |   ✅    | [Source](https://github.com/lagmoellertim/aoc-2025/tree/main/src/solver/day09) | [AoC](https://adventofcode.com/2025/day/9)  |
| 10  | Factory             |   ✅    |   ✅    | [Source](https://github.com/lagmoellertim/aoc-2025/tree/main/src/solver/day10) | [AoC](https://adventofcode.com/2025/day/10) |
| 11  | Reactor             |   ✅    |   ✅    | [Source](https://github.com/lagmoellertim/aoc-2025/tree/main/src/solver/day11) | [AoC](https://adventofcode.com/2025/day/11) |
| 12  | Christmas Tree Farm |   ✅    |   ✅    | [Source](https://github.com/lagmoellertim/aoc-2025/tree/main/src/solver/day12) | [AoC](https://adventofcode.com/2025/day/12) |

**Key:**

- ⬜ Not Started
- 🚧 In Progress
- ✅ Completed

## Author

**Tim-Luca Lagmöller** ([@lagmoellertim](https://github.com/lagmoellertim))

## Donations / Sponsors

I'm part of the official GitHub Sponsors program where you can support me on a monthly basis.

<a href="https://github.com/sponsors/lagmoellertim" target="_blank"><img src="https://github.com/lagmoellertim/shared-repo-files/raw/main/github-sponsors-button.png" alt="GitHub Sponsors" height="35px" ></a>

You can also contribute by buying me a coffee (this is a one-time donation).

<a href="https://ko-fi.com/lagmoellertim" target="_blank"><img src="https://github.com/lagmoellertim/shared-repo-files/raw/main/kofi-sponsors-button.png" alt="Ko-Fi Sponsors" height="35px" ></a>

Thank you for your support!

## License

The Code is licensed under the

[MIT License](https://github.com/lagmoellertim/aoc-2025/blob/master/LICENSE)

Copyright © 2025-present, [Tim-Luca Lagmöller](https://github.com/lagmoellertim)

## Have fun 🎉