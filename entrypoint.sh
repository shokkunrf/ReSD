#!/bin/sh
set -e

if [ -f ./package.json -a ! -d ./node_modules ]; then
  npm i
fi

if [ "${1#-}" != "${1}" ] || [ -z "$(command -v "${1}")" ]; then
  set -- node "$@"
fi

exec "$@"
