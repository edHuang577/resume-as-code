#!/usr/bin/env bash
set -e

if [ $# -lt 1 ]; then
  echo "Syntax: sync_s3.sh <s3-bucket>"
  exit 1
fi

BUCKET=$1

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )/.." && pwd )"
cd "$DIR"

if ! [[ $(aws --version) ]]; then
  echo "No 'aws' cli found for syncing to S3" >&2
  exit 1
fi

rm -rf dist/
mkdir dist/

cp wasm-app/{*.js,*.html,*.css,*.ico} dist/
cp *.pdf dist/
cp -r wasm-app/pkg/ dist/pkg/

aws s3 sync dist/ "s3://$BUCKET"

rm -rf dist/
