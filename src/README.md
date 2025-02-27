# Source code

# Intro

This project is using PyO3 and maturin, so you should know how to use those tools before reading this code.

# Structure

- **lib.rs**: It's like an "endpoint" to call our rust code from python
    - **WithOEL** and **WithCustomDelims** are implemented here
- **with_oel.rs**: It contains the class/methods that will be converted into python class/methods -> read each lines (separator = **\n**)
    - Available functions: **head**, **between**, **tail**, **parse**, **count_lines**
 - **with_custom_delims.rs**: the same behaviour as **with_oel.rs** but with a custom separator and not only **\n**
    - **utils/utils.rs**: it contains some function that are used by **WithOEL** and **WithCustomDelims**
    - **utils/test_utils.rs**: all utils for tests. Like array comparaison...
    - It's using **read_utf8** from [https://github.com/FlaveFlav20/ReadUTF8](https://github.com/FlaveFlav20/ReadUTF8) to read delimiters by delimiters with efficienty
