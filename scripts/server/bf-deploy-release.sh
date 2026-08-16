#!/usr/bin/env bash
# Install this file as root:root mode 0750 at /usr/local/sbin/bf-deploy-release.
# ```
# sudo install -o root -g root -m 0750 scripts/server/bf-deploy-release.sh /usr/local/sbin/bf-deploy-release
# ```
# It deliberately accepts only a fixed environment, commit SHA, and OCI image.
set -euo pipefail

if [ "$#" -ne 2 ]; then
  echo "usage: $0 <test|production> <commit-sha>" >&2
  exit 64
fi

environment="$1"
commit_sha="$2"

case "$environment" in
  test)
    compose_file=/etc/bouncy-feet/test/backend-compose.yaml
    backend_service=bouncy_backend_dev
    frontend_base=/services/app-frontend
    frontend_service=bf-dev
    health_url=http://127.0.0.1:3003/healthz
    state_dir=/var/lib/bouncy-feet/test
    ;;
  production)
    compose_file=/etc/bouncy-feet/production/backend-compose.yaml
    backend_service=bouncy_backend
    frontend_base=/services/app
    frontend_service=bf-prod-app
    health_url=http://127.0.0.1:3002/healthz
    state_dir=/var/lib/bouncy-feet/production
    ;;
  *)
    echo "invalid environment: $environment" >&2
    exit 64
    ;;
esac

[[ "$commit_sha" =~ ^[0-9a-f]{40}$ ]] || { echo "invalid commit SHA" >&2; exit 64; }

# This root-owned file prevents the SSH account from selecting an arbitrary
# image that Docker would run with the backend's runtime secrets.
backend_repository_file=/etc/bouncy-feet/backend-repository
[ -r "$backend_repository_file" ] || { echo "backend repository is not configured" >&2; exit 66; }
backend_repository="$(cat "$backend_repository_file")"
[[ "$backend_repository" =~ ^[a-zA-Z0-9./:_@-]+$ ]] || { echo "invalid configured backend repository" >&2; exit 66; }
backend_image="$backend_repository:$commit_sha"

deploy_user=deploy
archive="/var/tmp/bouncy-feet-frontend-${commit_sha}.tar.gz"
releases_dir="$frontend_base/releases"
live_link="$frontend_base/build"
release_id="$(date -u +%Y%m%d%H%M%S)-${commit_sha:0:12}"
release_dir="$releases_dir/$release_id"
previous_frontend=""
previous_image=""

[ -r "$archive" ] || { echo "frontend archive not found: $archive" >&2; exit 66; }
[ -r "$compose_file" ] || { echo "compose file not found: $compose_file" >&2; exit 66; }
id "$deploy_user" >/dev/null
install -d -m 0755 -o "$deploy_user" -g "$deploy_user" "$release_dir"
mkdir -p "$state_dir"

# The SSH account extracts the archive, so a compromised deployment archive is
# never interpreted as root-owned filesystem content.
runuser -u "$deploy_user" -- tar -xzf "$archive" -C "$release_dir"
rm -f "$archive"
test -f "$release_dir/build/index.js"
test -f "$release_dir/build/package.json"
test "$(cat "$release_dir/build/REVISION")" = "$commit_sha"

if [ -L "$live_link" ]; then
  previous_frontend="$(readlink -f "$live_link")"
fi
if [ -r "$state_dir/backend-image" ]; then
  previous_image="$(cat "$state_dir/backend-image")"
fi

# Preserve the existing Compose project name rather than deriving one from
# /etc/bouncy-feet/... and accidentally creating a second database/network.
compose() {
  docker compose --project-name bouncy_backend -f "$compose_file" "$@"
}

export BACKEND_IMAGE="$backend_image"
compose up -d postgres
compose pull "$backend_service"
# Migrations are intentionally a separate, visible deployment step. A new
# PostgreSQL container may need a few seconds before it accepts connections.
migrated=false
for _ in $(seq 1 15); do
  if compose run --rm "$backend_service" migrate; then
    migrated=true
    break
  fi
  sleep 2
done
if [ "$migrated" != true ]; then
  echo "database migration failed" >&2
  exit 1
fi
compose up -d --no-deps "$backend_service"

ready=false
for _ in $(seq 1 30); do
  if curl --fail --silent --show-error "$health_url" >/dev/null; then
    ready=true
    break
  fi
  sleep 2
done
if [ "$ready" != true ]; then
  echo "new backend did not become healthy" >&2
  if [ -n "$previous_image" ]; then
    BACKEND_IMAGE="$previous_image" compose up -d --no-deps "$backend_service" || true
  fi
  exit 1
fi

ln -sfn "$release_dir/build" "$live_link"
if ! systemctl restart "$frontend_service"; then
  echo "frontend restart failed; restoring the previous release" >&2
  if [ -n "$previous_frontend" ]; then
    ln -sfn "$previous_frontend" "$live_link"
    systemctl restart "$frontend_service" || true
  else
    rm -f "$live_link"
  fi
  if [ -n "$previous_image" ]; then
    BACKEND_IMAGE="$previous_image" compose up -d --no-deps "$backend_service" || true
  fi
  exit 1
fi

printf '%s\n' "$backend_image" > "$state_dir/backend-image"
printf '%s\n' "$commit_sha" > "$release_dir/build/BACKEND_REVISION"
# Keep the active frontend release and two previous releases.
mapfile -t old_releases < <(find "$releases_dir" -mindepth 1 -maxdepth 1 -type d -printf '%T@ %p\n' | sort -nr | tail -n +4 | cut -d ' ' -f2-)
if [ "${#old_releases[@]}" -gt 0 ]; then
  rm -rf -- "${old_releases[@]}"
fi
printf 'Deployed frontend and backend for %s (%s)\n' "$commit_sha" "$backend_image"
