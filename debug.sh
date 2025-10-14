#!/usr/bin/env bash

set -euxo pipefail

WEBSITE_NAME=testcase BIT_DEBUG=1 cargo run --quiet -r --bin bit &> testcase_bit.log
WEBSITE_NAME=testcase BIT_DEBUG=1 cargo run --quiet -r --bin tri &> testcase_tri.log
WEBSITE_NAME=testcase BIT_DEBUG=1 cargo run --quiet -r --bin quad &> testcase_quad.log
