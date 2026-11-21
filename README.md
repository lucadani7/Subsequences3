# Subsequences3

A Rust solution to [subsiruri3](https://infoarena.ro/problema/subsiruri3) from the infoarena archive (ONIS 2014, Round 4).

The problem statement belongs to its authors and to infoarena, so it is not copied here. See the link above for the full statement.

## The task in short

Given a sequence of integers and a value `K`, count the non-decreasing subsequences in which every step between consecutive chosen elements is at most `K`. The answer is printed modulo 666013.

## Approach

- Let `dp[i]` be the number of valid subsequences that end at position `i`. That subsequence is either `x[i]` alone, or a valid subsequence ending at an earlier `j` with `x[i] - K <= x[j] <= x[i]`, extended by `x[i]`.
- The answer is the sum of all `dp[i]`.
- Computing each `dp[i]` directly is O(N²), which is too slow for N up to 10⁶. Instead, the values are compressed to ranks and a Fenwick tree keeps the running sums of `dp` by value. Each step is then one range query and one point update.
- Input is parsed with a small buffered byte reader, because it can be tens of MB.

**Complexity:** O(N log N) time and O(N) memory per test.

## Building and running

Requires a recent stable Rust toolchain ([rustup.rs](https://rustup.rs)).

Like on infoarena, the program reads `subsiruri3.in` and writes `subsiruri3.out` in the current directory:

```bash
# create an input file with the sample test
printf "1\n4 2\n6 2 8 9\n" > subsiruri3.in

# build in release mode and run
cargo run --release

# check the result
cat subsiruri3.out   # expected: 7
```

Test files (`*.in`, `*.out`, `*.ok`) are ignored by git.

## Project layout

```
.
├── Cargo.toml
├── LICENSE
├── README.md
└── src/
    └── main.rs
```

## License

This project is licensed under the [Apache License 2.0](LICENSE).

The license covers the solution code only, not the problem statement.