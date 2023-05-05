# wordle-rs
commandline wordle written in rust
# setup
clone this repo to your computer, enter the newly created directory and type `cargo run`
# additional setup
by default `cargo run` builds the project without optimizations, so you may want to do the following:  
```
cargo build --release
```
and then add the following alias to your `.bashrc` file or `.zshrc` or whatever you have
```
aliase 'wordle=cargo run -- --release --manifest-path /your/path/to/wordle-rs/Cargo.toml'
```
use the entire path to your local directory and only use the --release flag if you built this project with the release flag.
