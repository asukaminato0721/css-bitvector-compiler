#!/usr/bin/env bash

set -euxo pipefail

WEBSITE_NAME=a_to_b BIT_DEBUG=1 cargo run --quiet -r --bin bit &> testcase_bit.log
WEBSITE_NAME=a_to_b BIT_DEBUG=1 cargo run --quiet -r --bin tri &> testcase_tri.log
# Quad remains available as an explicit experimental target, but is intentionally
# excluded from the default debug comparison until its semantics are validated.
