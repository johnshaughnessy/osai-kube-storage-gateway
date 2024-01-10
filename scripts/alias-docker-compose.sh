#!/bin/bash
#
# Alias for docker-compose, setting the correct compose file.
#
# Source this file: source alias-docker-compose.sh
#
# Then use the alias: dc ps, dc up, dc down, etc.

SCRIPT_DIR="$(dirname "$0")"
alias dc="docker compose -f $SCRIPT_DIR/docker-compose.dev.yaml"
