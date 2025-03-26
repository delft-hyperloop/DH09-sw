#!/bin/sh

echo 'Populating commands...'

cargo run -p goose_utils --bin output_gs_frontend -- ../config/dataflow.yaml