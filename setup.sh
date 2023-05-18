#! /bin/bash
## this script ensures that the use has cargo installed, 
# if the user does not have cargo installed, then dispaly error and return 1
# else attempt to create script /usr/local/bin/wordle
# if file already exists prompt user until user enters file that does not
# exist in that directory
# if the event that the user does not have /usr/local/bin directory, 
# this script will not work ##
if command -v cargo > /dev/null 2>&1
then
	echo >&2 'setting up wordle...'
else
	echo >&2 'please install cargo in order to use this app'
	exit 1

fi
cargo build --release
CURR_PATH="$(pwd)"
COMMAND='wordle'
while [[ -f "/usr/local/bin/${COMMAND}" ]]
do
	echo >&2 "/usr/local/bin/${COMMAND}" already exists
	echo >&2 'what do you want the command to be called?'
	read -r COMMAND
done
echo "!# /bin/bash
cd ${CURR_PATH}
cargo run --release" | sudo tee "${COMMAND}"
sudo chmod +x "${COMMAND}"
sudo mv "${COMMAND}" /usr/local/bin
