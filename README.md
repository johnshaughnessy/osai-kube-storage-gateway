# OSAI Kube Storage Gateway

Storage Gateway for use with [`osai-kube`](https://github.com/johnshaughnessy/osai-kube/), as described by [`this blog post`](https://www.johnshaughnessy.com/blog/posts/osai-kube-object-storage).

# Setup

Copy the example secrets to the secrets directory:

```sh
cp ./example-secrets/* secrets/
```

Edit the secrets to match your environment.

Run `./scripts/status.sh` to verify that your environment is properly configured.
