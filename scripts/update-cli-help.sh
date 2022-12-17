#!/usr/bin/env bash

THIS_SCRIPT_DIR=$(dirname $0)

FORMATTER_CMD="column -c"
COLUMNS=80

CLI_COMMANDS_YML=$(realpath "${THIS_SCRIPT_DIR}/../data/cli-commands.yml")
OUTPUT_DIR=$(realpath "${THIS_SCRIPT_DIR}/../embeds/cli/help")

yq eval '.cli-commands[]' $CLI_COMMANDS_YML | while read -r cmd; do
    printf '%s\n' "$cmd"

    # just call it `fluvio` and save as markdown
    FILENAME=$(printf "$cmd\n" | sed 's/fluvio-stable/fluvio/' | tr ' ' '-').md
    echo "\`\`\`" >$OUTPUT_DIR/$FILENAME
    eval "$cmd -h" | $FORMATTER_CMD $COLUMNS | sed 's/fluvio-stable/fluvio/' >>$OUTPUT_DIR/$FILENAME
    echo -n "\`\`\`" >>$OUTPUT_DIR/$FILENAME
done
