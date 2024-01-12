# OSAI Kube Storage Gateway

Storage Gateway for use with [`osai-kube`](https://github.com/johnshaughnessy/osai-kube/), as described by [`this blog post`](https://www.johnshaughnessy.com/blog/posts/osai-kube-object-storage).

# Setup

Copy the example secrets to the secrets directory:

```sh
cp ./example-secrets/* secrets/
```

Edit the secrets to match your environment.

Run `./scripts/status.sh` to verify that your environment is properly configured.

# Development

Storage gateway needs access to a real (remote) GCP bucket, but postgres can run anywhere (including locally). The easiest way to set this up is to use `docker compose` to run both `storage-gateway` and `postgres`, with volume mounts set up appropriately to get the live code editing / updating stuff working.

An even "easier" way would be to just run either (or both) apps on the host and forget about docker entirely, but I prefer not to "leak" things into my host dev environment if I can help it. Also, since these apps will eventually run in containers in prod, I need to do whatever work is required to declare runtime dependencies no matter what. I might as well do this for the development build as well.

If you `source ./scripts/alias-docker-compose.sh`, you can manage the dev environment with the usual docker compose commands (aliased to `dc`):

```sh
dc build
dc up -d
dc ps
dc logs -f
dc down
```

# Database setup and migrations

The database is not automatically created. Instead, a container called `migration-runner` is set up to do this. For first time, setup:

- Bring up the dev environment: `dc up -d`
- The `storage-gateway` logs will have errors about not being able to connect to the database. This is expected.
- Connect to the `migration-runner` container: `dc exec migration-runner bash`
- Inside `migration-runner`, run `source ./migration-runner-set-env.sh` to configure the `DATABASE_URL` environment variable.
- Inside `migration-runner`, run `diesel database setup` to create the database.
- Restart `storage-gateway` to get it to reconnect.

You can use `migration-runner` to run migrations manually. You may need to shut down `storage-gateway` first, because `postgres` may disallow certain operations if there are active connections to the database.

After bringing up the dev environment (`dc up -d`), connect to the `migration-runner` container and run:
