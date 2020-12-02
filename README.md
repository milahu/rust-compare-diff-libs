# rust-compare-diff-libs

comparison of diff algorithms in rust

* Longest Common Subsequence (LCS) algorithm
* Myers diff algorithm

### todo

* read input from files, like the [diffr](https://crates.io/crates/diffr) tool
   * add more tests to compare algos, like [diff-slider-tools/corpus](https://github.com/mhagger/diff-slider-tools)
* compare lines / tokens / bytes
* Patience diff algorithm
   * implemented in `diffs` library
   * much slower than myers algorithm
   * can fix 'slider errors' = wrong grouping of lines / code blocks
* Histogram diff algorithm
   * can be faster than myers algorithm
   * does not work on all inputs (limited number of different tokens?)
   * no rust implementation yet?
   * java: [jgit/diff/HistogramDiff.java](https://github.com/eclipse/jgit/blob/master/org.eclipse.jgit/src/org/eclipse/jgit/diff/HistogramDiff.java)
   * C: [git/xdiff/xhistogram.c](https://github.com/git/git/blob/master/xdiff/xhistogram.c)
* include [other libraries](https://crates.io/keywords/diff)
   * https://crates.io/crates/bsdiff
   * https://crates.io/crates/seqdiff
   * https://crates.io/crates/diffr-lib
   * https://crates.io/crates/differ
   * https://crates.io/crates/diffus
   * https://crates.io/crates/line_diff
   * ....

### binary file size

```
$ ./run.sh

file size
total   extra  file

293024      0  target/release/noop-str
293024      0  target/release/noop-string
309408  16384  target/release/diffs-myers-byte
309408  16384  target/release/diffs-myers-token
313504  20480  target/release/difference-lcs
333984  40960  target/release/dissimilar-myers
```

### reduce binary size

https://github.com/johnthagen/min-sized-rust

`strip` will reduce size by 70%

```
cargo build --release
strip target/release/*
```

removing panic/format also brings
[drastic reductions](https://jamesmunns.com/blog/fmt-unreasonably-expensive/)
in file size (this is important for embedded targets),
but libraries must be patched

### related

* https://github.com/Wilfred/difftastic
   * [diff-slider-tools/corpus](https://github.com/mhagger/diff-slider-tools)
   * [drastic reductions](https://jamesmunns.com/blog/fmt-unreasonably-expensive/) in file size
