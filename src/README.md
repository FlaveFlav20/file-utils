# Source code

# Intro

This project is using PyO3 and maturin, so you should know how to use those tools before reading this code.

# Structure

- **lib.rs**: It's like an "endpoint" to call our rust code from python
    - Only **WithOEL** is implemented here
- **with_oel.rs**: It contains the class/methods that will be converted into python class/methods -> read each lines (separator = **\n**)
    - Available functions: **head**, **between**, **tail**, **parse**, **count_lines**
 - **with_custom_delims.rs**: the same behaviour as **with_oel.rs** but with a custom separator and not only **\n** -> it's not implemented because it's complicated to test...
    - **read_delim.rs** is a custom "readlines" but between a list of chosen separator.
    - **utils.rs**: it only contains a VecQueue to Vec converter. 