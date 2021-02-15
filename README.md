# Wireworld

## Intro

A implementation of the [wireworld](https://en.wikipedia.org/wiki/Wireworld) cellular automaton proposed by Brian Silverman in 1987. Written in [Rust](https://www.rust-lang.org/) using [SDL2](https://www.libsdl.org/index.php) as the multimedia engine.

<p align="center">
  <img width="450" height="450" src="https://raw.githubusercontent.com/ghikio/wireworld/master/.images/main.png"
       alt="2 clock generators sending electrons into an XOR gate in wireworld">
</p>

## Deps

The only dependency is [SDL2](https://www.libsdl.org/index.php) for which you require it's libraries. In most Linux distributions you can install them with the `SDL2-devel` package or similar.

## Running it

In order to run it you only need a working [Rust](https://www.rust-lang.org/) environment:

```sh
cd wireworld
cargo run # Runs it using the development profile
# or
cargo run --release # Runs it using the release profile which applies some optimizations
```

After the window opens up, `Left Clicking` any cell will produce the following output:

+ If the cell is `Empty` (black), it will turn it into a `Conductor` (yellow).
+ If the cell is a `Conductor` (yellow), it will turn it into a `Electron Head` (blue).

In order to make a cell `Empty` (black) again, you can `Right Click` it.

For more info on how the automaton works, check the [wikipedia page](https://en.wikipedia.org/wiki/Wireworld) :)
