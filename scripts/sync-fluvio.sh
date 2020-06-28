#!/bin/bash
BRANCH=${1:-"master"}

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
SCRIPT="$DIR/sync-repo.sh"

RUN="${SCRIPT} -b ${BRANCH} -r fluvio -d cli"

echo $RUN
$RUN