### Advent of Code 2022 - in Rust!

Playing around and learning some Rust features a bit better.

As of right now the plan is to make all the days challenge solvers act like a CLI tool, where the 'puzzle inputs' should be piped in, and the final result will be the only output, ie:

```
## from within one of the `day-{n}` subdirs:
cargo build
cat input/input | ./target/build/debug/day-{n}
```

