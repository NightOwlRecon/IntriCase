#!/bin/sh
source .env
psql -d "${DATABASE_URL}" -f seed.sql
