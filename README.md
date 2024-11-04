# file operations

## Intro

This package allows to read/parse a file in python. When should we use this package? If your file is really big (> 100 000 lines), beacause if you want to parse a file in python you'll write:
```py
f = open("my_file", "r")
buffer: str = f.read()
...
```
or:
```py
f = open("my_file", "r")
for line in f.readlines():
    ...
```
- With the first one, there is a memory issue beacause you must save the entire file into a buffer. 
- With the second one, there is a time issue because a loop can be very slow in python.

So, this package gives tools to easily read a file with efficienty. It's based on linux tools like **grep**, **sed**, **cat** and tested with them.

# Example

```py

import file_operations_lib

path: str = "my_path_to_file"
n: int = 10 # Number of lines to read

try:
    head: list = file_operations_lib.WithEOL.head(file=path, n=n)
    print(head)
except:
    print("Unable to open/read the file)
```

## Python structure

If we translate the rust into python, we'll have:
```py
class WithEOL:
    def head(file: str, n: int, \
                remove_empty_string: bool = False, \
                keep_when_regex: bool = False, \
                pass_when_regex: bool = False, \
                regex_keep: list = [] \
                regex_pass: list = [] \
                restrict: bool = True)

    def between(file: str, n1: int, n2: int \
                remove_empty_string: bool = False, \
                keep_when_regex: bool = False, \
                pass_when_regex: bool = False, \
                regex_keep: list = [] \
                regex_pass: list = [] \
                restrict: bool = True)
    
    def tail(file: str, n: int, \
                remove_empty_string: bool = False, \
                keep_when_regex: bool = False, \
                pass_when_regex: bool = False, \
                regex_keep: list = [] \
                regex_pass: list = [] \
                restrict: bool = True)
    
    def parse(file: str, \ 
                remove_empty_string: bool = False \
                keep_when_regex: bool = False \
                pass_when_regex: bool = False \
                regex_keep: list = [] \
                regex_pass: list = [])

    def count_lines(file: str \
                    remove_empty_string: bool = False, \
                    keep_when_regex: bool = False \
                    pass_when_regex: bool = False \
                    regex_keep: list = [] \
                    regex_pass: list = [])
```

## Arguments explaination

- **file**: the path to the file
- **remove_empty_string**: igonore the empty string **"[ ]\*"**
- **n**: get n lines with **tail**/**head** 
- **n1**: the begin line to take with **between**
- **n2**: the last line to take with **between**
- **restrict**: if enbla, if we have last N lines, it just keep the regex in thoses lines. If not enable, it takes last N regex

with **regex**:
- **keep_when_regex**: if enable, it ignore other lines that maching with the regex
- **pass_when_regex**: if anable, it ignore the lines that maching with regex
- **regex_keep**: list of regex to keep if **keep_when_regex** is enable
- **regex_pass**: list of regex to pass if **pass_when_regex** is enable

## Structure

- **src/**: all srources files
- **tests/**: all tests for rust
- **tests_files/**: all files used to tests
- **tests_python/**: a python script to test