# Forgejo deployment setup

The deployment workflow builds the frontend and the backend OCI image from the
same Git SHA. Forgejo stores the backend image in its OCI registry; the target
server pulls that immutable SHA tag. The workflow uploads the frontend archive
to the server, then invokes the root-owned deployment helper to migrate, start
and health-check the backend before switching the frontend release.

## Forgejo runner

Use an amd64 Linux Forgejo runner with access to a Docker daemon. It needs Docker
BuildKit, Node 22, Rust/cargo, and permission to build and push OCI images. The
backend image must be built for the same architecture as the target hosts; the
current deployment assumes amd64.

## Forgejo secrets

Set these repository secrets. A Forgejo access token is preferable to a user
password.

| Secret | Purpose |
| --- | --- |
| `REGISTRY_HOST` | Registry hostname, for example `code.jakobmeier.ch` |
| `BACKEND_REPOSITORY` | Full untagged image name, for example `code.jakobmeier.ch/bouncy-feet/bouncy-backend` |
| `REGISTRY_USERNAME` | Account allowed to write that package |
| `REGISTRY_TOKEN` | Token with package write access |
| `DEV_DEPLOY_SSH_KEY`, `DEV_DEPLOY_KNOWN_HOSTS`, `DEV_DEPLOY_TARGET`, `DEV_DEPLOY_PORT` | Test-host SSH access |
| `PROD_DEPLOY_SSH_KEY`, `PROD_DEPLOY_KNOWN_HOSTS`, `PROD_DEPLOY_TARGET`, `PROD_DEPLOY_PORT` | Production-host SSH access |

Use separate SSH keys for test and production. `*_DEPLOY_TARGET` is in
`user@host` form; `*_DEPLOY_KNOWN_HOSTS` is the literal known_hosts entry, not
the output of an unchecked `ssh-keyscan` in CI. Protect `bf`, and configure the
Forgejo `production` environment to require approval.

## Target-server prerequisites

Each target needs Docker Engine with the Compose v2 plugin, `curl`, `sudo`, and
`acl` (for `setfacl`). On Debian, after Docker itself is installed:

```sh
sudo apt update
sudo apt install acl curl
sudo docker compose version
```

The deployment helper also uses the standard `runuser` command supplied by
Debian's `util-linux` package. Ensure the frontend systemd service names and
backend host ports match its `test`/`production` cases before the first deploy.

## Server secrets and configuration

Do **not** put runtime secrets in Forgejo or in the Compose files. On each
server, create root-owned configuration. Substitute the real values below.

```sh
sudo install -d -m 700 /etc/bouncy-feet/test /etc/bouncy-feet/production
sudoedit /etc/bouncy-feet/test/backend.env
sudoedit /etc/bouncy-feet/test/postgres.env
sudoedit /etc/bouncy-feet/production/backend.env
sudoedit /etc/bouncy-feet/production/postgres.env
sudo chmod 600 /etc/bouncy-feet/{test,production}/*.env
sudo chown root:root /etc/bouncy-feet/{test,production}/*.env
```

`backend.env` contains the backend runtime variables, including secrets:

```dotenv
CLIENT_URL=https://dev.bouncy-feet.ch
API_URL=https://dev-api.bouncy-feet.ch
QR_TRUSTED_TARGET_ORIGINS=
PEERTUBE_URL=https://dev-tube.bouncy-feet.ch
OIDC_ISSUER=https://auth.bouncy-feet.ch/realms/bouncyfeet-dev
OIDC_CLIENT_ID=dev-backend
OIDC_CLIENT_SECRET=replace-me
DATABASE_URL=postgres://api_user:replace-me@postgres/bouncyfeet-dev
PEERTUBE_SYSTEM_USER=system
PEERTUBE_SYSTEM_USER_PASSWORD=replace-me
```

`postgres.env` contains only the database container bootstrap values:

```dotenv
POSTGRES_DB=bouncyfeet-dev
POSTGRES_USER=api_user
POSTGRES_PASSWORD=replace-me
```

Use production URLs, realm/client names, and database values in the production
files. The password in `DATABASE_URL` and `POSTGRES_PASSWORD` must agree.

Install the version-controlled non-secret Compose files once on each server. The
helper deliberately uses Compose project name `bouncy_backend`, matching the
existing directory-based project name; verify it with `sudo docker compose ls`
before the first automated deployment.


```sh
sudo install -m 600 -o root -g root bouncy_backend/docker-compose.dev-hosted.yaml \
  /etc/bouncy-feet/test/backend-compose.yaml
sudo install -m 600 -o root -g root bouncy_backend/docker-compose.hosted.yaml \
  /etc/bouncy-feet/production/backend-compose.yaml
```

Configure the one backend repository that the helper is permitted to run. This
is root-owned configuration, not an argument accepted from the SSH account:

```sh
printf '%s\n' 'code.jakobmeier.ch/bouncy-feet/bouncy-backend' | \
  sudo tee /etc/bouncy-feet/backend-repository >/dev/null
sudo chown root:root /etc/bouncy-feet/backend-repository
sudo chmod 600 /etc/bouncy-feet/backend-repository
```

## Restricted deployment account

Create a dedicated SSH account named `deploy`. It needs normal write access to
`/var/tmp`, but must not be in the `docker` group and must not have general sudo
access. The root-owned helper creates the release directory and has `deploy`
extract the archive, so the archive is never unpacked as root. Install the
helper as root. `deploy` needs only traversal access to the frontend release
path; it does not need to modify the live symlink, systemd, Docker,
`/etc/bouncy-feet`, or `/root/.docker`.

```sh
sudo install -o root -g root -m 0750 scripts/server/bf-deploy-release.sh \
  /usr/local/sbin/bf-deploy-release
sudo setfacl -m u:deploy:--x /services /services/app-frontend \
  /services/app-frontend/releases
```

`/var/tmp` is normally mode `1777` and is the only ordinary writable location
used by the SSH account: the workflow uploads its frontend archive there. The
helper creates each release directory as `deploy`, extracts the archive as that
user, then performs privileged activation itself.

Allow only the needed helper invocation in `/etc/sudoers.d/bouncy-feet-deploy`.
Use a separate production account/rule if possible:

```sudoers
Cmnd_Alias BF_DEPLOY_TEST = /usr/local/sbin/bf-deploy-release ^test [0-9a-f]{40}$
deploy ALL=(root) NOPASSWD: BF_DEPLOY_TEST
```

The workflow uploads the frontend archive to
`/var/tmp/bouncy-feet-frontend-<sha>.tar.gz` and invokes that helper. The helper
runs the explicit `migrate` command, pulls the pinned backend image, waits for
`/healthz`, then restarts the frontend. It records the prior image and restores
it if backend startup or frontend restart fails. Database migrations themselves
must therefore be backwards-compatible; they cannot be automatically rolled
back safely.
