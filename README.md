# wordle-rs
commandline wordle written in rust
# setup
clone this repo to your computer, enter the newly created directory and type `cargo run`
# additional optional setup
## optimization
by default `cargo run` builds the project without optimizations, so you may want to do the following:  
```
cargo build --release
```
## play wordle from any directory
this part was tricky for me to figure out but here is what I came up with.

create the following script called `wordle` and place it anywhere in your `PATH` (I recommend `/usr/local/bin`)  
```
#! /bin/bash
cd <PATH_TO_YOUR_REPO>/wordle-rs
cargo run --release             
```
use the absolute path for `<PATH_TO_YOUR_REPO>` and only use `--release` if your also followed the above step.  
then finally make the script executable with:  
```
sudo chmod +x wordle
```
now you can run `wordle` from any directory.


