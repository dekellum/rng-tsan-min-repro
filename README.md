# rng-tsan-min-repro

Minimized reproductions of a thread sanitizer issues (see various branches)

Steps to reproduce with (rust/LLVM) thread sanitizer:

``` bash
export TSAN_OPTIONS="suppressions=`pwd`/tsan"
RUST_BACKTRACE=1 RUSTFLAGS="-Z sanitizer=thread" cargo test
```

If this is a false positive, this can be added to the suppressions file, but so
far seems under-specific:

``` txt
# See https://github.com/dekellum/rng-tsan-min-repro
race:lazy_static::lazy::Lazy<T>::get
```
