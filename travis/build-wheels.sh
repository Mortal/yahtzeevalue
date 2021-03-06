#!/bin/sh
# Based on https://github.com/getsentry/symbolic
# and https://github.com/pypa/python-manylinux-demo

set -e -x

# Install dependencies needed by our wheel
yum -y install gcc libffi-devel

# Install Rust
curl https://sh.rustup.rs -sSf | sh -s -- -y
export PATH=~/.cargo/bin:$PATH

# Build wheels
cd /work
/opt/python/cp27-cp27mu/bin/python setup.py bdist_wheel --verbose

# Audit wheels
for wheel in dist/*-linux_*.whl; do
  auditwheel repair $wheel -w wheelhouse/
done
