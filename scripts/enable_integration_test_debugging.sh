#!/bin/sh

# Copy resource files into the target directory so that integration tests can
# be run in the debugger.
#
# You must first create the build directory with `cargo run` or `cargo test`.


REPO_ROOT=$(git rev-parse --show-toplevel)

# Ensure target directory exists
mkdir -p ${REPO_ROOT}/target/debug/tests

# Remove existing/outdated files
rm -f ${REPO_ROOT}/target/debug/tests/*

# Copy current contents of ./tests into target directory
cp -r ${REPO_ROOT}/tests/*.txt ${REPO_ROOT}/target/debug/tests/

# Echo results
FILES=${REPO_ROOT}/target/debug/tests/*.txt
echo "Created the following files:"
for f in $FILES; do
    echo "$f"
done
