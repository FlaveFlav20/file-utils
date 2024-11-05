# file operations

## Intro

This package allows to read/parse a file in python. When should we use this package? If your file is really big (> 100 000 lines), because if you want to parse a file in python you'll write:
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
- With the first one, there is a memory issue because you must save the entire file into a buffer. 
- With the second one, there is a time issue because a loop can be very slow in python.

So, this package gives tools to easily read a file with efficiently. It's based on Linux tools like **grep**, **sed**, **cat**, **head**, **tail** and tested with them.

## Example

```py

import file_operations_lib

path: str = "my_path_to_file"
n: int = 10 # Number of lines to read

try:
    head: list = file_operations_lib.WithEOL.head(path=path, n=n)
    print(head)
except:
    print("Unable to open/read the file")
```

## Python class

If we translate the rust into python, we'll have:
```py
class WithEOL:
    def head(path: str, n: int, \
                remove_empty_string: bool = False, \
                regex_keep: list = [] \
                regex_pass: list = [] \
                restrict: bool = True):
        ...

    def between(path: str, n1: int, n2: int \
                remove_empty_string: bool = False, \
                regex_keep: list = [] \
                regex_pass: list = [] \
                restrict: bool = True):
        ...
    
    def tail(path: str, n: int, \
                remove_empty_string: bool = False, \
                regex_keep: list = [] \
                regex_pass: list = [] \
                restrict: bool = True):
        ...
    
    def parse(path: str, \ 
                remove_empty_string: bool = False \
                regex_keep: list = [] \
                regex_pass: list = []):
        ...

    def count_lines(path: str \
                    remove_empty_string: bool = False, \
                    regex_keep: list = [] \
                    regex_pass: list = []):
        ...
```

## Arguments explaination

- **path**: the path to the file
- **remove_empty_string**: ignore the empty string **"[ ]\*"**
- **n**: get n lines with **tail**/**head** 
- **n1**: the beginning line to take with **between**
- **n2**: the last line to take with **between**
- **restrict**: if enable, if we have last N lines, it just keep the regex in those lines. If not enable, it takes last N regex

with **regex**:
- **regex_keep**: list of regex to keep
- **regex_pass**: list of regex to pass/ignore

## Structure

- **src/**: all sources files
- **tests/**: all tests for rust
- **tests_files/**: all files used for tests
- **tests_python/**: a python script to test