# LeetCode in Rust

Rust solutions to LeetCode problems, organized as standalone Cargo binaries.

## Requirements

- Rust stable toolchain

## Run a solution

Each solution has a small `main` function with an example input. Run one with:

```bash
cargo run --bin <solution>
```

For example:

```bash
cargo run --bin ex_time
```

## Solutions

| LeetCode problem | Binary | Main idea |
| --- | --- | --- |
| [1929. Concatenation of Array](https://leetcode.com/problems/concatenation-of-array/) | `array_i` | Slice concatenation |
| [448. Find All Numbers Disappeared in an Array](https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/) | `dissappeared_nums` | Frequency counting |
| [636. Exclusive Time of Functions](https://leetcode.com/problems/exclusive-time-of-functions/) | `ex_time` | Stack-based time accounting |
| [485. Max Consecutive Ones](https://leetcode.com/problems/max-consecutive-ones/) | `max_con` | Single pass |
| [150. Evaluate Reverse Polish Notation](https://leetcode.com/problems/evaluate-reverse-polish-notation/) | `polish_notation` | Operand stack |
| [645. Set Mismatch](https://leetcode.com/problems/set-mismatch/) | `set_mismatch` | Frequency counting |
| [1470. Shuffle the Array](https://leetcode.com/problems/shuffle-the-array/) | `shuffle_array` | Interleaving |
| [1365. How Many Numbers Are Smaller Than the Current Number](https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/) | `smaller` | Pairwise comparison |
| [1441. Build an Array With Stack Operations](https://leetcode.com/problems/build-an-array-with-stack-operations/) | `stack` | Push/pop simulation |

## Verify the project

```bash
cargo fmt -- --check
cargo test --all-targets
```

The solutions use only the Rust standard library.
