#!/bin/bash

# Source this file to set the DATABASE_URL environment variable

# File path to the config.yaml
CONFIG_FILE="/etc/storage-gateway/config.yaml"

# Extract values from the YAML file
DATABASE_HOST=$(awk '/database_host:/ {print $2}' $CONFIG_FILE)
DATABASE_NAME=$(awk '/database_name:/ {print $2}' $CONFIG_FILE)
DATABASE_USERNAME=$(awk '/database_username:/ {print $2}' $CONFIG_FILE)
DATABASE_PASSWORD=$(awk '/database_password:/ {print $2}' $CONFIG_FILE)

# Construct the DATABASE_URL
DATABASE_URL="postgres://${DATABASE_USERNAME}:${DATABASE_PASSWORD}@${DATABASE_HOST}/${DATABASE_NAME}"

echo "DATABASE_URL: ${DATABASE_URL}"

# Export the DATABASE_URL environment variable
export DATABASE_URL

# Now you can run diesel commands
# diesel migration run
