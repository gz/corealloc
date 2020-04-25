# corealloc

Figures out which core-ids to pin your program to:

Allocate 12 cores, as close together as possible:

```bash
cargo run --release -- -c 12 -s sequential
```

Allocate 12 cores, spread out over multiple sockets, if possible:

```bash
cargo run --release -- -c 12 -s interleave
```

Same but don't consider neighbouring hyper-threads for allocation:

```bash
cargo run --release -- -c 12 -s interleave --no-ht
```
