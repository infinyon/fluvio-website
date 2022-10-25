#!/usr/bin/env bash

THIS_SCRIPT_DIR=$(dirname $0)

CLI_COMMANDS_YML=$(realpath "${THIS_SCRIPT_DIR}/../data/cli-commands.yml")
OUTPUT_DIR=$(realpath "${THIS_SCRIPT_DIR}/../code-blocks/cli/help")

yq eval '.cli-commands[]' $CLI_COMMANDS_YML | while read -r cmd; do
    printf '%s\n' "$cmd"

    # just call it `fluvio`
    FILENAME=$(printf "$cmd\n" | sed 's/fluvio-stable/fluvio/' | tr ' ' '-').txt
    eval "$cmd -h" | sed 's/fluvio-stable/fluvio/' >$OUTPUT_DIR/$FILENAME
done
