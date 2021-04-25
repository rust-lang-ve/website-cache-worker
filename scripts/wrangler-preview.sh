#!/bin/bash

RUST_BACKTRACE=1 RUST_LOG=error,info,warn,all npx @cloudflare/wrangler preview --watch
