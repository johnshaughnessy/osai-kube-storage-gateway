#!/usr/bin/env sh

SCRIPT_DIR="$(dirname "$0")"
source "$SCRIPT_DIR/set_environment_variables.sh"
source "$SCRIPT_DIR/log.sh" # log

log "WARN" "This script will delete the dev database and all data in the pgdata directory."

read -p "Are you sure you want to continue? (y/n) " -n 1 -r
echo

# If user does not say yes, exit
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    log "INFO" "Exiting."
    exit 1
fi

log "INFO" "Deleting database..."
sudo rm -rf "$SCRIPT_DIR/../pgdata"

mkdir "$SCRIPT_DIR/../pgdata"

log "INFO" "Database deleted."
