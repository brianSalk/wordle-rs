# wordle-rs
multi-lingual command-line wordle written in rust!  
The language is based on the environment variable `LANG`, so if you want to play wordle in german, set `LANG` to `de`  
# setup
clone this repo to your computer, enter the newly created directory and type `cargo run` to play command-line wordle.  
to create a global `wordle` command, run `setup.sh`    
this will put an executable command with the default name `wordle` in your `/usr/local/bin` directory  
now you can just type `wordle` into your terminal from anywhere and play wordle!  
# requirements
In order to run this application, you will need to have some type of UNIX-shell (BASH, Z-SHELL etc.) and you  
must also have `cargo` installed.  Cargo can be installed from all the major package managers.  Running the `setup.sh` script will attempt to install cargo on your version of linux.






