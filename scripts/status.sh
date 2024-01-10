#!/usr/bin/env sh
#
# Validate that we have read/write access to the GCS bucket

SCRIPT_DIR="$(dirname "$0")"
source "$SCRIPT_DIR/set_environment_variables.sh"
source "$SCRIPT_DIR/log.sh" # log

print_usage() {
    echo "
Usage:

    status.sh [options]

Options:
"
}

# Parse command line arguments
for arg in "$@"; do
    if [ "$arg" = "--help" ]; then
        print_usage
        exit 0
    elif [ "$arg" = "help" ]; then
        print_usage
        exit 0
    elif [ "$arg" = "h" ]; then
        print_usage
        exit 0
    fi
done

result=$(gsutil ls gs://$GCS_BUCKET_NAME)
# If the result is empty, we don't have access
if [ -z "$result" ]; then
    log "ERROR" "Could not access GCS bucket gs://$GCS_BUCKET_NAME"
    exit 1
fi

log "OK" "Verified access to GCS bucket gs://$GCS_BUCKET_NAME"
