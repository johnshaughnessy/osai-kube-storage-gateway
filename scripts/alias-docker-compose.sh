#!/bin/bash
#
# Function for docker-compose, setting the correct compose file.
#
# Source this file: source alias-docker-compose.sh
#
# Then use the function: dc ps, dc up, dc down, etc.

SCRIPT_DIR="$(dirname "$0")"

function dc {
    HOST_UID=$(id -u)
    HOST_GID=$(id -g)
    docker compose -f "$SCRIPT_DIR/docker-compose.dev.yaml" "$@"
}
