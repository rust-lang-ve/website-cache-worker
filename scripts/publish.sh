#!/bin/bash

# This script relies on having CF_EMAIL and CF_API_TOKEN (Global API Key)
# environment variables defined. Make sure the wrangler.toml file contains
# the right `account_id` value.
#
# Read more here: https://developers.cloudflare.com/workers/cli-wrangler/authentication#using-environment-variables

# CF_EMAIL=youremail@provider.com \
# CF_API_KEY=yourglobalapikey \
RUST_BACKTRACE=1 \
RUST_LOG=error,info,warn,all npx @cloudflare/wrangler publish
