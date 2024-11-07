# file operations

## Table of contents

- [Intro](#Intro)
- [Python class](#Python-class)
- [Arguments-explaination](#Arguments-explaination)
- [Structure](#Structure)

Examples:
- [Example-file](#Example-file)
- Examples:
    - [Example-simple-head](#Example-simple-head)
    - [Example-simple-tail](#Example-simple-tail)
    - [Example-simple-between](#Example-simple-between)
    - [Example-simple-parse](#Example-simple-parse)
    - [Example-simple-count_lines](#Example-simple-count_lines)
    - [Example-remove_empty_string](#Example-remove_empty_string)
    - [Example-regex_keep](#Example-regex_keep)
    - [Example-regex_pass](#Example-regex_pass)
    - [Example-restrict](#Example-restrict)

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

## Python-class

If we translate the rust into python, we'll have:
```py
class WithEOL:
    # head: Read the n first lines
    # if n > (numbers of lines in the file) => return the whole file
    def head(path: str, n: int, \
                remove_empty_string: bool = False, \
                regex_keep: list = [] \
                regex_pass: list = [] \
                restrict: bool = True):
        ...

    # between: Read the lines [n1, n2]
    # if n1 > n2 => return an empty list
    # if n1 > (numbers of lines in the file) => return an empty list
    def between(path: str, n1: int, n2: int \
                remove_empty_string: bool = False, \
                regex_keep: list = [] \
                regex_pass: list = [] \
                restrict: bool = True):
        ...
    
    # tail: Read the n last lines
    # if n > (numbers of lines in the file) => return the whole file
    def tail(path: str, n: int, \
                remove_empty_string: bool = False, \
                regex_keep: list = [] \
                regex_pass: list = [] \
                restrict: bool = True):
        ...
    
    # parse: Read the whole file
    def parse(path: str, \ 
                remove_empty_string: bool = False \
                regex_keep: list = [] \
                regex_pass: list = []):
        ...

    # Count the number of lines
    def count_lines(path: str \
                    remove_empty_string: bool = False, \
                    regex_keep: list = [] \
                    regex_pass: list = []):
        ...
```

## Arguments-explaination

- **path**: the path to the file
- **remove_empty_string**: ignore the empty string **"[ ]\*"**
- **n**: get n lines with **tail**/**head** 
- **n1**: the beginning line to take with **between**
- **n2**: the last line to take with **between**
- **restrict**: if enable, if we have last N lines, it just keep the regex in those lines. If not enable, it takes last N regex

with **regex**:
- **regex_keep**: list of regex to keep
- **regex_pass**: list of regex to pass/ignore

## Example-file

We will use this example file **test.txt**

With **cat -e test.txt**:

```txt
[Warning]:Entity not found$
[Error]:Unable to recover data$
[Info]:Segfault$
[Warning]:Indentation$
[Error]:Memory leaks$
[Info]:Entity not found$
[Warning]:Unable to recover data$
  $
[Error]:Segfault$
[Info]:Indentation$
[Warning]:Memory leaks$
 ```

## Example-simple-head

1\ Simple head (can be change to tail)
Code:
```py

import file_utils_operations_lib

path: str = "my_path_to_file"
n: int = 2 # Number of lines to read

try:
    head: list = file_utils_operations_lib.WithEOL.head(path=path, n=n)
    print(head)
except:
    print("Unable to open/read the file")
```
Stdout:
```sh
['[Warning]:Entity not found', '[Error]:Unable to recover data']
```

## Example-simple-tail

Code:
```py

import file_utils_operations_lib

path: str = "my_path_to_file"
n: int = 2 # Number of lines to read

try:
    tail: list = file_utils_operations_lib.WithEOL.tail(path=path, n=n)
    print(tail)
except:
    print("Unable to open/read the file")
```
Stdout:
```sh
['[Info]:Indentation', '[Warning]:Memory leaks']
```

## Example-simple-between

Code:
```py

import file_utils_operations_lib

path: str = "my_path_to_file"
n1: int = 2 # First line to read
n2: int = 4 # Last line to read

try:
    between: list = file_utils_operations_lib.WithEOL.between(path=path, n1=n1, n2=n2)
    print(between)
except:
    print("Unable to open/read the file")
```
Stdout:
```sh
['[Error]:Unable to recover data', '[Info]:Segfault', '[Warning]:Indentation']
```

## Example-simple-parse

Code:
```py

import file_utils_operations_lib

path: str = "my_path_to_file"

try:
    between: list = file_utils_operations_lib.WithEOL.parse(path=path)
    print(between)
except:
    print("Unable to open/read the file")
```
Stdout:
```sh
['[Warning]:Entity not found', '[Error]:Unable to recover data', '[Info]:Segfault', '[Warning]:Indentation', '[Error]:Memory leaks', '[Info]:Entity not found', '[Warning]:Unable to recover data', '  ', '[Error]:Segfault', '[Info]:Indentation', '[Warning]:Memory leaks']
```

## Example-simple-count_lines

Code:
```py

import file_utils_operations_lib

path: str = "my_path_to_file"

try:
    between: list = file_utils_operations_lib.WithEOL.count_lines(path=path)
    print(between)
except:
    print("Unable to open/read the file")
```
Stdout:
```sh
11
```

## Example-remove_empty_string

With **remove_empty_string** enable: \
Code:
```py

import file_utils_operations_lib

path: str = "my_path_to_file"
n: int = 4 # First line to read

try:
    tail: list = file_utils_operations_lib.WithEOL.tail(path=path, n=n, remove_empty_string=True)
    print(tail)
except:
    print("Unable to open/read the file")
```
Stdout:
```sh
['[Warning]:Unable to recover data', '[Error]:Segfault', '[Info]:Indentation', '[Warning]:Memory leaks']
```

With **remove_empty_string** disable (default option): \
Code:
```py

import file_utils_operations_lib

path: str = "my_path_to_file"
n: int = 4 # First line to read

try:
    tail: list = file_utils_operations_lib.WithEOL.tail(path=path, n=n, remove_empty_string=False)
    print(tail)
except:
    print("Unable to open/read the file")
```
Stdout:
```sh
['  ', '[Error]:Segfault', '[Info]:Indentation', '[Warning]:Memory leaks']
```

## Example-regex_keep

Code:
```py

import file_utils_operations_lib

path: str = "my_path_to_file"
n: int = 4 # First line to read

try:
    head: list = file_utils_operations_lib.WithEOL.head(path=path, n=n, remove_empty_string=False, regex_keep=["\[Warning\]:*", "\[Error\]:*"])
    print(head)
except:
    print("Unable to open/read the file")
```
Stdout:
```sh
['[Warning]:Entity not found', '[Error]:Unable to recover data', '[Warning]:Indentation']
```

Why tere is just 3 elements instead of 4? You should look at the **restrict** option

## Example-regex_pass

Code:
```py

import file_utils_operations_lib

path: str = "my_path_to_file"
n: int = 4 # First line to read

try:
    head: list = file_utils_operations_lib.WithEOL.head(path=path, n=n, remove_empty_string=False, regex_pass=["\[Warning\]:*", "\[Error\]:*"])
    print(head)
except:
    print("Unable to open/read the file")
```
Stdout:
```sh
['[Info]:Segfault']
```

Why tere is just 3 elements instead of 4? You should look at the **restrict** option

## Example-restrict

With **restrict** disable: \
Code:
```py

import file_utils_operations_lib

path: str = "my_path_to_file"
n: int = 4 # First line to read

try:
    head: list = file_utils_operations_lib.WithEOL.head(path=path, n=4, remove_empty_string=False, regex_keep=["\[Warning\]:*", "\[Error\]:*"], restrict=False)
    print(head)
except:
    print("Unable to open/read the file")
```
Stdout:
```sh
['[Warning]:Entity not found', '[Error]:Unable to recover data', '[Warning]:Indentation', '[Error]:Memory leaks']
```

With **restrict** enbale(default): \
Code:
```py

import file_utils_operations_lib

path: str = "my_path_to_file"
n: int = 4 # First line to read

try:
    head: list = file_utils_operations_lib.WithEOL.head(path=path, n=4, remove_empty_string=False, regex_keep=["\[Warning\]:*", "\[Error\]:*"], restrict=True)
    print(head)
except:
    print("Unable to open/read the file")
```
Stdout:
```sh
['[Warning]:Entity not found', '[Error]:Unable to recover data', '[Warning]:Indentation']
```

## Structure

- **src/**: all sources files
- **tests/**: all tests for rust
- **tests_files/**: all files used for tests
- **tests_python/**: a python script to test