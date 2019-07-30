# rng-tsan-min-repro

Minimized reproduction of a thread sanitizer issues, some fixed via use of
pcg32.

Steps to reproduce with (rust/LLVM) thread sanitizer:

``` bash
export TSAN_OPTIONS="suppressions=`pwd`/tsan"
RUST_BACKTRACE=1 RUSTFLAGS="-Z sanitizer=thread" cargo test
```
