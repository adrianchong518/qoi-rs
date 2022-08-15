# qoi-rs

Another implementation of the [QOI image format](https://qoiformat.org) in pure,
safe Rust.

This is only a hobby project that I took up for fun, do not expect any
"production-quality" code here. The *QOI Format* was very interesting to me and
looks very simple given its performance.

## Building

If you are fancy and have `nix`, you build the project with

```bash
nix build
nix run
```

Or with `cargo` using

```bash
cargo build
cargo run
```

## Targets

- [ ] Fully implement encoding and decoding by the
      [specification](https://qoiformat.org/qoi-specification.pdf)
  - [x] Encoding
  - [ ] Decoding
- [ ] Create a CLI tool to convert to and from different image file formats
  - [ ] PNG
  - [ ] TIFF
