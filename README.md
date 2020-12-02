# rust-compare-diff-libs

comparison of diff algorithms in rust

* Longest Common Subsequence (LCS) algorithm
* Myers diff algorithm

### todo

* Patience diff algorithm
* Histogram diff algorithm

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

### related

https://github.com/Wilfred/difftastic

[jgit/HistogramDiff.java](https://github.com/eclipse/jgit/blob/master/org.eclipse.jgit/src/org/eclipse/jgit/diff/HistogramDiff.java)

### reduce binary size

https://github.com/johnthagen/min-sized-rust

https://jamesmunns.com/blog/fmt-unreasonably-expensive/

`strip` will reduce size by 70%

```
cargo build --release
strip target/release/*
```
