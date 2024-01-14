#!/bin/bash
set -e

create_user_and_db() {
    local user=$1
    local pass=$2
    local dbname=$3

    echo "Creating user $user and database $dbname..."

    local create_db_sql
    create_db_sql+="CREATE ROLE \"$user\" WITH LOGIN CREATEDB PASSWORD '$pass'; "
    echo "$create_db_sql" | psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER"
}

STORAGE_GATEWAY_USER="storage_gateway_role"
STORAGE_GATEWAY_PASS="password"
STORAGE_GATEWAY_DB="storage_gateway_db"
create_user_and_db $STORAGE_GATEWAY_USER $STORAGE_GATEWAY_PASS $STORAGE_GATEWAY_DB
