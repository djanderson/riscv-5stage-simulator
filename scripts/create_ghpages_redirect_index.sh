#!/bin/sh

set -e

# rustdoc puts resource files at root level and the package's index.html file 1
# level deeper, but then navigating to a repo's default gh-pages link returns a
# 404. Work around that by writing a redirect index file to the doc root.
#
# You must first create the doc directory with `cargo doc`.


REPO_ROOT=$(git rev-parse --show-toplevel)

REDIRECT='<meta HTTP-EQUIV="REFRESH" content="0; url=riscv_5stage_simulator/index.html">'

echo $REDIRECT > ${REPO_ROOT}/target/doc/index.html
