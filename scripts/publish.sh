#!/bin/bash

curl -sv -X PUT "https://api.cloudflare.com/client/v4/accounts/$CF_ACCOUNT_ID/workers/scripts/rusty-cloudflare-worker" \
  -H "X-Auth-Email:$CF_EMAIL" \
  -H "X-Auth-Key:$CF_API_KEY" \
  -F 'metadata=@worker/metadata_wasm.json;type=application/json' \
  -F 'wasmprogram=@pkg/rusty_cloudflare_worker_bg.wasm;application/wasm' \
  -F 'script=@pkg/rusty_cloudflare_worker.js;type=application/javascript' \
  -F 'script=@worker/worker.js;type=application/javascript'
