#!/bin/bash

SCRIPT_DIR="$(dirname "$0")"
source "$SCRIPT_DIR/set_environment_variables.sh"
source "$SCRIPT_DIR/log.sh"

PRINT_USAGE_AND_EXIT=0
DOCKER_BUILD=0
DOCKER_DOWN=0
DOCKER_UP=0
# Make sure that only one command is specified
COMMAND_COUNT=0

# Parse command line arguments
for arg in "$@"; do
    if [ "$arg" = "--help" ]; then
        PRINT_USAGE_AND_EXIT=1
    elif [ "$arg" = "-h" ]; then
        PRINT_USAGE_AND_EXIT=1
    elif [ "$arg" = "help" ]; then
        PRINT_USAGE_AND_EXIT=1
        COMMAND_COUNT=$((COMMAND_COUNT + 1))
    elif [ "$arg" = "build" ]; then
        DOCKER_BUILD=1
        COMMAND_COUNT=$((COMMAND_COUNT + 1))
    elif [ "$arg" = "down" ]; then
        DOCKER_DOWN=1
        COMMAND_COUNT=$((COMMAND_COUNT + 1))
    elif [ "$arg" = "up" ]; then
        DOCKER_UP=1
        COMMAND_COUNT=$((COMMAND_COUNT + 1))
    else
        log "ERROR" "Unknown command: $arg"
        PRINT_USAGE_AND_EXIT=1
    fi
done

if [ $COMMAND_COUNT -gt 1 ]; then
    log "ERROR" "Only one command can be specified"
    PRINT_USAGE_AND_EXIT=1
fi

# If no command was specified, and no other options were specified, print usage
if [ $COMMAND_COUNT -eq 0 ] && [ $PRINT_USAGE_AND_EXIT -eq 0 ]; then
    PRINT_USAGE_AND_EXIT=1
fi

if [ $PRINT_USAGE_AND_EXIT -eq 1 ]; then
    echo "
Usage:

        dev.sh [options] [command]

Options:

        --help, -h      Print usage

Commands:

        build           Build Docker images
        down            Stop Docker containers
        help            Print usage
        up              Start Docker containers

"
    exit 0
fi

if [ $DOCKER_BUILD -eq 1 ]; then
    log "INFO" "Building Docker images..."
    docker-compose -f $SCRIPT_DIR/docker-compose.dev.yaml build
fi

if [ $DOCKER_DOWN -eq 1 ]; then
    log "INFO" "Stopping Docker containers..."
    docker-compose -f $SCRIPT_DIR/docker-compose.dev.yaml down
fi

if [ $DOCKER_UP -eq 1 ]; then
    log "INFO" "Starting Docker containers..."
    docker-compose -f $SCRIPT_DIR/docker-compose.dev.yaml up -d
fi
