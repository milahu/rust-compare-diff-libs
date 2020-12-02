#!/bin/bash

cargo build --release

strip target/release/* 2>/dev/null

# noop-str and noop-string have same size
noop_size=$(du -b target/release/noop-str | awk '{ print $1 }' )

printf '\nfile size\ntotal   extra  file\n\n'

find target/release/ -maxdepth 1 -type f -executable -exec du -b '{}' \; \
| sort -n \
| awk "{ printf \"%6s  %5s  %s\n\", \$1, (\$1 - $noop_size), \$2 }"

