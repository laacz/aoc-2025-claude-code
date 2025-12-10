# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Advent of Code 2025 solutions in Rust. Each day's puzzle has two parts solved in a single source file.

## Commands

```bash
# Run a specific day's solution
cargo run --bin day01

# Run tests for a specific day
cargo test day01

# Run all tests
cargo test
```

## Architecture

- `src/dayXX.rs` - Solution file for each day (binary target)
- `data/XX.txt` - Puzzle input files
- `tasks/dayXX-Y.md` - Task descriptions (part 1 or 2)

Each day file follows this pattern:
- `main()` - Reads input from `data/XX.txt`, calls both parts
- `parse()` - Converts input string to appropriate data structure
- `part1()` / `part2()` - Solution functions
- `tests` module - Unit tests using example input from task description

## Workflow for Solving a Day

1. Read task from `tasks/dayXX-Y.md`
2. Read input from `data/XX.txt`
3. Create/modify `src/dayXX.rs`
4. Add binary target to `Cargo.toml`:
   ```toml
   [[bin]]
   name = "dayXX"
   path = "src/dayXX.rs"
   ```
5. Write tests using example from task, run with `cargo test dayXX`
6. Run solution with `timeout 30 cargo run --bin dayXX`

## Output Format

Print only the final answer: `partY: RESULT`

## Guidelines

- Parse input from string parameter, don't hardcode file paths in solution functions
- Extract example input/output from task description for tests
- Prefer simple, readable solutions over clever ones
- Part 2 often reuses Part 1 logic - copy into `part2` function and refactor if needed
- If tests pass but answer is wrong, check for off-by-one or integer overflow
