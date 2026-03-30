# Building judges
Install `cargo install cross`

Then run `cross build -r --target x86_64-unknown-linux-musl`.

The binaries (`exact`, `tiny`, `heuristic`, `lower`) are placed in `target/x86_64-unknown-linux-musl`

# Scores
Depending on the track, the judges output different scores:
- Tiny: solution size (int)
- Exact: solution size
- Heuristic: solution size
- LowerBound: runtime (in ms)