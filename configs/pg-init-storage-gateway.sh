#!/bin/bash
set -e

create_user_and_db() {
    local user=$1
    local pass=$2
    local dbname=$3

    echo "Creating user $user, database $dbname, and granting permissions..."

    # Create user and database
    local create_db_sql
    create_db_sql+="CREATE ROLE \"$user\" WITH LOGIN CREATEDB PASSWORD '$pass'; "
    # create_db_sql+="CREATE DATABASE \"$dbname\"; "
    # create_db_sql+="GRANT ALL PRIVILEGES ON DATABASE \"$dbname\" TO \"$user\";"
    # create_db_sql+="ALTER DATABASE \"$dbname\" OWNER TO \"$user\";"
    echo "$create_db_sql" | psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER"

    echo "Creating schema and tables..."
    # Grant permissions in the newly created database
    # local grant_perms_sql
    # grant_perms_sql+="GRANT ALL PRIVILEGES ON SCHEMA public TO \"$user\";"

    # echo "$grant_perms_sql" | psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" -d "$dbname"
}

STORAGE_GATEWAY_USER="storage_gateway_role"
STORAGE_GATEWAY_PASS="password"
STORAGE_GATEWAY_DB="storage_gateway_db"
create_user_and_db $STORAGE_GATEWAY_USER $STORAGE_GATEWAY_PASS $STORAGE_GATEWAY_DB
