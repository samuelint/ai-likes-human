#!/bin/bash

#!/bin/bash

NEXT_VERSION=$1


jq --arg NEXT_VERSION "$NEXT_VERSION" '.package.version = $NEXT_VERSION' src-tauri/tauri.conf.json > tmpfile && mv tmpfile src-tauri/tauri.conf.json

