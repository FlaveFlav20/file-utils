# file utils: Rust tests

## Intro

These tests have been tested on Linux (ubutu)

## How to run tests?

To run tests, you should execute these commands:
1) You must create test files
```sh
python3 tests_files/create_test_custom_delims_file.py
```
2) Then, you should run tests with
```sh
cargo test
```

## Tests format

In the tests folder, we'll have those files:

```
test_"method to test"_"function to test".rs
```

The **_EOL** is just a custom_delim test with the delimiter "**\n**"