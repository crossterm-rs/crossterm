[![GitHub Actions][s1]][l1] [![MIT][s2]][l2]

# Crossterm Examples

The examples are compatible with the latest release.

## Structure

```
├── examples
│   └── interactive-demo
│   └── event-*
│   └── stderr
```

| File Name                   | Description                     | Topics                                    |
|:----------------------------|:--------------------------------|:------------------------------------------|
| `examples/interactive-demo` | interactive, walk through, demo | cursor, style, event                      |
| `event-*`                   | event reading demos             | (async) event reading                     |
| `stderr`                    | Crossterm over stderr demo      | raw mode, alternate screen, custom output |

## Run examples

```bash
$ cargo run --example [file name]
```

To run the interactive demo go into the folder `examples/interactive-demo` and run `cargo run`.

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

[s1]: https://github.com/crossterm-rs/crossterm/actions/workflows/ci.yml/badge.svg
[l1]: https://github.com/crossterm-rs/crossterm/actions

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: ../LICENSE
