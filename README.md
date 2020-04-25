# corealloc

Figures out which core-ids to pin your program to.

## Installation

```bash
cargo install corealloc
```

## Usage

Assume two sockets:

```log
Socket 0: ( 0 28 1 29 2 30 3 31 4 32 5 33 6 34 7 35 8 36 9 37 10 38 11 39 12 40 13 41 )
Socket 1: ( 14 42 15 43 16 44 17 45 18 46 19 47 20 48 21 49 22 50 23 51 24 52 25 53 26 54 27 55 )
```

```bash
corealloc -c 12 -t sequential
0 1 2 3 4 5 6 7 8 9 10 11 12 13 28 29 30 31 32 33 34 35 36 37 38 39 40 41 14 15 16 17 18 19 20 21 22 23 24 25 26 27 42 43 44 45 46 47 48 49 50 51 52 53 54 55 
```

Allocate 12 cores, spread out over multiple sockets, if possible:

```bash
corealloc -c 12 -t interleave
0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 
```

Same but don't consider neighbouring hyper-threads for allocation:

```bash
corealloc -c 28 -t sequential --no-ht
0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27
```

```bash
corealloc -c 28 -t interleave --no-ht
0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27
```
