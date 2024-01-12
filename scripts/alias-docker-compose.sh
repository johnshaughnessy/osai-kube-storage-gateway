#!/bin/bash
#
# Function for docker-compose, setting the correct compose file.
#
# Source this file: source alias-docker-compose.sh
#
# Then use the function: dc ps, dc up, dc down, etc.

SCRIPT_DIR="$(dirname "$0")"
CONFIG_DIR="$SCRIPT_DIR/../configs"

function dc {
    HOST_UID=$(id -u)
    HOST_GID=$(id -g)
    docker compose -f "$CONFIG_DIR/docker-compose.dev.yaml" "$@"
}
