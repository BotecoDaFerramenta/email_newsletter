#!/usr/bin/env bash

cargo watch -x check -x test -x run --ignore tmp | tee ./tmp/logs/log_file.txt
