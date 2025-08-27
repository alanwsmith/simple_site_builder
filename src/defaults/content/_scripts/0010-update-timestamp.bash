#!/bin/bash

TIMESTAMP=$(date "+%B %-d, %Y")

echo "{ \"timestamp\": \"$TIMESTAMP\" }" > "../_data/updated.json"
