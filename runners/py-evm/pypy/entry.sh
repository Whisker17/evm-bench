#!/usr/bin/env bash
set -eo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cd "$SCRIPT_DIR"
export POETRY_INSTALLER_NO_BINARY="${POETRY_INSTALLER_NO_BINARY:-ckzg,lru-dict}"
export PIP_NO_BINARY="${PIP_NO_BINARY:-ckzg,lru-dict}"
poetry env use pypy3 >&2
poetry sync >&2
if ! poetry run python - <<'PY' >/dev/null 2>&1
import ckzg
import lru
PY
then
  poetry run python -m pip install --force-reinstall --no-binary=:all: ckzg lru-dict >&2
fi
poetry run python ../runner.py "$@"
