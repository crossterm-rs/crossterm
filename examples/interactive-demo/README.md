# Crossterm Interactive Demo

An interactive terminal application for demonstrating and manually testing Crossterm functionality.

The demo includes tests for:

* cursor movement and visibility;
* foreground and background colors;
* text attributes;
* terminal input events;
* synchronized output.

## Running the demo

From the root of the Crossterm repository, run:

```sh
cargo run --manifest-path examples/interactive-demo/Cargo.toml
```

The demo must be run in an interactive terminal.

## Controls

From the main menu:

* `1` — test cursor functionality
* `2` — test foreground and background colors
* `3` — test text attributes
* `4` — test input events
* `5` — test synchronized output
* `q` — return to the main menu or exit the demo

During most tests, press any other key to continue to the next step.
