#!/bin/bash

WORKER_NAME="website-cache"
CF_ACCOUNT_ID="<Provide your Account ID from Cloudflare Workers Website>"
WORKERS_DEV=true

if [[ $1 == "prod" ]] ; then
  echo "Using production configuration"
  WORKERS_DEV=false
fi

echo "Creating wrangler.toml file for \"$WORKER_NAME\""
echo -e "name = '$WORKER_NAME'\ntype = 'rust'\n\naccount_id = '$CF_ACCOUNT_ID'\nworkers_dev = $WORKERS_DEV\nzone_id = '$CF_ZONE_ID'\nroute = '$ROUTE'" > wrangler.toml