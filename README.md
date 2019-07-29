# rng-tsan-min-repro

Minimized reproduction of a thread sanitizer issue on getrandom 0.1.7:

Steps to reproduce with (rust/LLVM) thread sanitizer:

``` bash
export TSAN_OPTIONS="suppressions=`pwd`/tsan"
RUST_BACKTRACE=1 RUSTFLAGS="-Z sanitizer=thread" cargo test
```

To make the error go away, downgrade getrandom crate:

``` bash
cargo update -p getrandom --precise 0.1.6
```

Or this can be added to the suppressions file, but so far seems under-specific:

``` txt
# See https://github.com/dekellum/rng-tsan-min-repro
race:lazy_static::lazy::Lazy<T>::get
```
