# #!/bin/bash
#
# Source this file to use the log function.

log() {
    local prefix="osai-kube"
    local status="$1"
    local message="$2"

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
    elif [ "$status" = "INFO" ]; then
        # Gray (37)
        status_with_color="\033[0;37m$1\033[0m"
    else
        # Gray (37)
        status_with_color="\033[0;37m$1\033[0m"
    fi

    local indent_string=""
    if [ ${#status} -lt 5 ]; then
        indent_string=$(printf "%$((5 - ${#status}))s")
    fi

    echo -e "[$prefix] [$status_with_color] $indent_string $message"
}
