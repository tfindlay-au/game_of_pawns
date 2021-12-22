# SHOKUNIN 2019 April Challenge: Game of Pawns

![rust](https://www.rust-lang.org/static/images/rust-logo-blk.svg)

## Purpose

Generate a random chess position as an 8x8 grid or in FEN format. The position does not have to be realistic, but it must follow these rules:

 - there is one and only one king of each color;
 - the kings must not be placed on adjacent squares;
 - there can not be any pawn in the promotion square (no white pawn in the eighth rank, and no black paw - in the first rank);
 - including the kings, up to 32 pieces of either color can be placed. There is no requirement for material balance between sides, but the picking of pieces should comply with what is found in a regular chess set (e.g., 8 pawns/colour, 1 queen/colour, etc)
 - (FEN requirement only) it is white's turn, both sides have lost castling rights and there is no possibility for en passant (the FEN should thus end in w - - 0 1)

## Assumptions

### 1
` - the kings must not be placed on adjacent squares`

 If the kings are never placed -/+ 10 cells around each other, it's ok.

### 2
Acceptable command-line arguments:

`--grid` - will output a board in a 8x8 grid format

`all the other input or the absence of it` - will output a board in a fen format

## How to run (please check `Prerequisites`)

```
./go.sh --grid
```

expected output:
```
 .  .  .  Q  .  .  b  .
 .  .  .  .  .  .  .  .
 R  .  .  b  .  .  r  .
 .  .  .  .  q  .  .  K
 .  .  .  .  P  .  .  .
 .  B  k  P  .  .  .  .
 .  n  .  .  .  .  .  .
 .  .  .  N  .  .  .  .
```

```
./go.sh --fen
```

expected output:
```
3Q2b1/8/R2b2r1/4q2K/4P3/1BkP4/1n6/3N4/ w - - 0 1
```

## Prerequisites

### Install rust and cargo
```
curl https://sh.rustup.rs -sSf | sh
```

## How to dev and tests

To run the tests:

```
cargo test
```

To run:

```
cargo run
```