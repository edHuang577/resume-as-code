#!/usr/bin/env bash
set -e

OUTFILE=${1:-Resume.pdf}

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )/.." && pwd )"
cd "$DIR"

if [[ "$OSTYPE" == "darwin"* ]]; then
    shopt -s expand_aliases
    alias chrome="/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome"
fi

if ! [[ $(chrome --version) ]]; then
  echo "No 'chrome' command found for running headless PDF generation" >&2
  exit 1
fi

if ! [[ $(python --version) ]]; then
  echo "No 'python' command found for running headless PDF generation" >&2
  exit 1
fi

rm -rf dist/
mkdir dist/

cp wasm-app/{*.js,*.html,*.css,*.ico} dist/
cp -r wasm-app/pkg/ dist/pkg/

rm -rf "$OUTFILE"

cd dist/
python -m http.server 8081 &
PID=$!
cd ..

sleep 1

{
    chrome --headless --disable-gpu \
        --run-all-compositor-stages-before-draw \
        --print-to-pdf="$OUTFILE" \
        http://localhost:8081

} || {
    printf ""
}

kill "$PID"
rm -rf dist/
