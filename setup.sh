#! /bin/bash
## this script ensures that the user has cargo installed, 
# if the user does not have cargo installed, then dispaly error and return 1
# else attempt to create script /usr/local/bin/wordle
# if file already exists prompt user until user enters file that does not
# exist in that directory
# in the event that the user does not have /usr/local/bin directory, 
# this script will not work ##
if command -v cargo > /dev/null 2>&1
then
	echo >&2 'setting up wordle...'
else
	sudo apt-get install cargo 2>/dev/null || sudo dnf install cargo 2>/dev/null || sudo yum install cargo 2>/dev/null || sudo pacman -Sy cargo 2>/dev/null
	if [[ $? -eq 0 ]]
	then
		echo -e "\033[32mCARGO INSTALLED SUCCESSFULLY\033[0m"
	else
		echo -e >&2 "\033[1;31mSETUP FAILED\033[0m"
		echo >&2 'please install cargo in order to use this app'
		exit 1
	fi
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
# write file to /usr/local/bin/{command}
echo "!# /bin/bash
cd ${CURR_PATH}
cargo run" | sudo tee "${COMMAND}" > /dev/null
sudo chmod +x "${COMMAND}"
sudo mv "${COMMAND}" /usr/local/bin
echo -e >&2 "\033[32;40mYou are all set!\033[0m"
echo -e >&2 "To play wordle, type \033[1m${COMMAND}\033[0m in your terminal"
