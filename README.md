# LobsTeX
A mini LaTeX clone prototype in Rust.

## Compilation and usage
To build the project, launch at the root of the repository:
```bash
cargo build
```
You can launch LobsTeX using
```bash
./target/debug/lobstex demo.lob
```
to compile any LobsTeX file to a PDF.

Alternatively, you can do:
```bash
cargo run demo.lob
```
which builds and executes LobsTeX.

## How it works
This project is build in Rust. A simili-LaTeX file is lexed and parsed using [`lalrpop`](https://github.com/lalrpop/lalrpop) to build a simple AST. This AST is then rendered into a PDF using - for now - the [`genpdf`](https://github.com/dabega/genpdf-rs) crate. I might transition from the high-level `genpdf` crate towards the lower-level `printpdf` crate to add more features, such as justified alignment.

## TODO
- Page numbers
- Margin handling
- Bold, italic text
- Justified alignment