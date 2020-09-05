This is a test case to show the difference between Rust 1.45 and 1.46
in terms of how a particular type of error is reported when it is
generated from within an attribute macro.

To demonstrate:

* rustup install 1.45.0
* rustup install 1.46.0
* cargo +1.45.0 run --example test
* cargo +1.46.0 run --example test


I know we are all *extremely* tired of 45, and nothing could be more welcome than 46, but in this corner case, 1.45 is slightly better.
