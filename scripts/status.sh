#!/usr/bin/env sh
#
# Validate that we have read/write access to the GCS bucket

prefix="osai-kube"

# Use prefix to print messages.
# Make sure each message is on a new line,
# and is indented based on the length of the status.
# For example:
#  [osai-kube] [OK]    Message 1
#  [osai-kube] [ERROR] Message 2
log() {
    local status="$1"
    local message="$2"

    # Create a version of the status that also prints in color
    local status_with_color=""
    if [ "$status" = "OK" ]; then
        # Green (32)
        status_with_color="\033[0;32m$1\033[0m"
    elif [ "$status" = "ERROR" ]; then
        # Red (31)
        status_with_color="\033[0;31m$1\033[0m"
    elif [ "$status" = "WARN" ]; then
        # Yellow (33)
        status_with_color="\033[0;33m$1\033[0m"
    else
        # Gray (37)
        status_with_color="\033[0;37m$1\033[0m"
    fi

    local indent_string=""
    # If the status is less than 5 characters, add spaces to indent
    # the message by 5 - length(status) spaces
    if [ ${#status} -lt 5 ]; then
        indent_string=$(printf "%$((5 - ${#status}))s")
    fi

    # Print the message (including the indentation and status)
    echo -e "[$prefix] [$status_with_color] $indent_string $message"

}

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

SCRIPT_DIR="$(dirname "$0")"
source "$SCRIPT_DIR/set_environment_variables.sh"

result=$(gsutil ls gs://$GCS_BUCKET_NAME)
# If the result is empty, we don't have access
if [ -z "$result" ]; then
    log "ERROR" "Could not access GCS bucket gs://$GCS_BUCKET_NAME"
    exit 1
fi

log "OK" "Verified access to GCS bucket gs://$GCS_BUCKET_NAME"
