#! /bin/bash
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