#!/usr/bin/env bash

THIS_SCRIPT_DIR=$(dirname $0)

FORMATTER_CMD="column -c"
COLUMNS=80

CLI_COMMANDS_YML=$(realpath "${THIS_SCRIPT_DIR}/../data/cli-commands.yml")
OUTPUT_DIR=$(realpath "${THIS_SCRIPT_DIR}/../embeds/cli/help")

yq eval '.cli-commands[]' $CLI_COMMANDS_YML | while read -r cmd; do
    printf '%s\n' "$cmd"

    # just call it `fluvio` and save as markdown
    FILENAME=$(printf "$cmd\n" | sed 's/fluvio-stable/fluvio/' | sed 's/fluvio-latest/fluvio/' | tr ' ' '-').md
    echo "\`\`\`" >$OUTPUT_DIR/$FILENAME

    # Run the command help option
    # Pass through formatter to constrain column width
    # Replace `fluvio-stable` with just `fluvio`
    # Make default paths match the linux defaults
    eval "$cmd -h" |
        $FORMATTER_CMD $COLUMNS |
        sed -e 's/fluvio-stable/fluvio/' -e 's/fluvio-latest/fluvio/' -e 's|/usr/local/var/log/fluvio|/tmp|' \
            >>$OUTPUT_DIR/$FILENAME
    echo -n "\`\`\`" >>$OUTPUT_DIR/$FILENAME
done
