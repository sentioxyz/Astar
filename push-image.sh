# VERSION=$(git describe --tags --abbrev=0)

set -e

BRANCH=$(git branch --show-current)
VERSION=${BRANCH#release/}

echo "VERSION: $VERSION"

docker build -t ghcr.io/sentioxyz/astar:latest -t ghcr.io/sentioxyz/astar:$VERSION -f third-party/docker/Dockerfile . --push
