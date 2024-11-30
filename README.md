# file utils

## Table of contents

- [Intro](#Intro)
- [Installation](#Installation)
    - [python](#python)
    - [rust](#rust)
- [Before starting](#Before-starting)
- [Arguments-explaination](#Arguments-explaination)

Examples:
- **WithEOL python**:
    - [Example-file](#Example-file)
    - Examples:
        - [Example-simple-head](#Example-simple-head-python)
        - [Example-simple-tail](#Example-simple-tail-python)
        - [Example-simple-between](#Example-simple-between-python)
        - [Example-simple-parse](#Example-simple-parse-python)
        - [Example-simple-count_lines](#Example-simple-count_lines-python)
        - [Example-remove_empty_string](#Example-remove_empty_string-python)
        - [Example-regex_keep](#Example-regex_keep-python)
        - [Example-regex_pass](#Example-regex_pass-python)
        - [Example-restrict](#Example-restrict-python)
- **WithCustomDelims**:
    - [How to use it?](#How-to-use-it-python)
    - [What delim can be used?](#What-delim-can-be-used-python)
    - [With more than one delimiter?](#With-more-than-one-delimiter-python)

- [Python class](#Python-class)
- [Rust Structure](#Rust-Structure)
- [Structure](#Structure)

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

So, this package gives tools to easily read a file with efficiently. It's based on Linux tools like **grep**, **sed**, **cat**, **head**, **tail** and tested with them. \
**WithEOL** class as the same memory problem as the first example. If you want to resolve it, you must use **WithCustomDelims** with the **"\n"** delimiter. \
So, why I keep **WithEOL**? \
**WithEOL** is helping me to test the code, it's using a built in rust function and I'm using it as a reference to compare with **WithCustomDelims**.

## Installation

### python

With **pypi**:
```sh
pip install file-utils-operations
```

From source:
```sh
maturin develop
```

### rust

```sh
cargo add file_utils_operations
```

## Before-starting

This package is ASCII/UTF-8 compliant, all others encoded files will not work...

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

## WithEOL-python

### Example-file

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

### Example-simple-head-python

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

### Example-simple-tail-python

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

### Example-simple-between-python

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

### Example-simple-parse-python

Code:
```py
import file_utils_operations_lib

path: str = "my_path_to_file"

try:
    parse: list = file_utils_operations_lib.WithEOL.parse(path=path)
    print(parse)
except:
    print("Unable to open/read the file")
```
Stdout:
```sh
['[Warning]:Entity not found', '[Error]:Unable to recover data', '[Info]:Segfault', '[Warning]:Indentation', '[Error]:Memory leaks', '[Info]:Entity not found', '[Warning]:Unable to recover data', '  ', '[Error]:Segfault', '[Info]:Indentation', '[Warning]:Memory leaks']
```

### Example-simple-count_lines-python

Code:
```py
import file_utils_operations_lib

path: str = "my_path_to_file"

try:
    count: list = file_utils_operations_lib.WithEOL.count_lines(path=path)
    print(count)
except:
    print("Unable to open/read the file")
```
Stdout:
```sh
11
```

### Example-remove_empty_string-python

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

### Example-regex_keep-python

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

Why there is just 3 elements instead of 4? You should look at the **restrict** option

### Example-regex_pass-python

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

Why there is just 3 elements instead of 4? You should look at the **restrict** option

### Example-restrict-python

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

## WithCustomDelims-python

### How-to-use-it-python

It it like **WithEOL** but with a list of custom delimiter. For example:

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

has the same behavious as 

```py
import file_utils_operations_lib

path: str = "my_path_to_file"
n: int = 2 # Number of lines to read

try:
    head: list = file_utils_operations_lib.WithCustomDelims.head(path=path, n=n, delimiter=['\n])
    print(head)
except:
    print("Unable to open/read the file")
```
Stdout:
```sh
['[Warning]:Entity not found', '[Error]:Unable to recover data']
```

So, you use it as same as **WithEOL** but with a list of custom delimiter.

### What-delim-can-be-used

All string can be used like:
- ";"
- "abc"
- "éà"
- ::
- "小六号"
- "毫" 

### With-more-than-one-delimiter

If my file contains:
```sh
;À ;la ;;
pêche éèaux moules, @moules, ::小六号moules::Je n'veux小六号 plus ::y 
aller éèmaman小六号
```

We'll have with ";", "\n", "éè", "@", "小六号", "::"
```py
import file_utils_operations_lib

path: str = "my_path_to_file"

try:
    parse: list = file_utils_operations_lib.WithCustomDelims.parse(path=path, delimiter=[";", "\n", "éè", "@", "::"])
    print(parse)
except:
    print("Unable to open/read the file")
```

Stdout

```sh
['', 'À ', 'la ', '', '', 'pêche ', 'aux moules, ', 'moules, ', '', 'moules', "Je n'veux", ' plus ', 'y ', 'aller ', 'maman', '']
```

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

class WithCustomDelims:
    # head: Read the n first lines
    # if n > (numbers of lines in the file) => return the whole file
    def head(path: str, n: int, delimiter: list \
                remove_empty_string: bool = False, \
                regex_keep: list = [] \
                regex_pass: list = [] \
                restrict: bool = True \
                buffer_size: int = 1024):
        ...

    # between: Read the lines [n1, n2]
    # if n1 > n2 => return an empty list
    # if n1 > (numbers of lines in the file) => return an empty list
    def between(path: str, n1: int, n2: int, delimiter: list \
                remove_empty_string: bool = False, \
                regex_keep: list = [] \
                regex_pass: list = [] \
                restrict: bool = True \
                buffer_size: int = 1024):
        ...
    
    # tail: Read the n last lines
    # if n > (numbers of lines in the file) => return the whole file
    def tail(path: str, n: int, delimiter: list \
                remove_empty_string: bool = False, \
                regex_keep: list = [] \
                regex_pass: list = [] \
                restrict: bool = True \
                buffer_size: int = 1024):
        ...
    
    # parse: Read the whole file
    def parse(path: str, delimiter: list \
                remove_empty_string: bool = False \
                regex_keep: list = [] \
                regex_pass: list = [] \
                buffer_size: int = 1024):
        ...

    # Count the number of lines
    def count_lines(path: str, delimiter: list \
                    remove_empty_string: bool = False, \
                    regex_keep: list = [] \
                    regex_pass: list = [] \
                    buffer_size: int = 1024):
        ...
```

## Rust-Structure

```rs
...
impl WithEOL {
    ///
    /// [WithEOL]\[head\]: take the first n lines. \
    /// Arguments:
    /// - path (mandatory): to the file
    /// - n (mandatory): number of lines
    /// - remove_empty_string (optional): remove all string that only contains
    /// spaces
    /// - regex_keep (optional): a list of regex to keep
    /// - regex_pass (optional): a list of regex to pass
    /// - restrict (optional): if enable, it will only take the first n lines.
    /// If not, it will take the first n lines that can be taken (you can take
    /// a look at the README to have further explaination)
    /// 
    /// return:
    /// - A list of string 
    /// 
    pub fn head(
        path: String,
        n: usize,
        remove_empty_string: bool,
        regex_keep: Vec<String>,
        regex_pass: Vec<String>,
        restrict: bool,
    ) -> Vec<String> {
        ...
    }

    ///
    /// [WithEOL]\[between\]: take the lines between n1 and n2 \
    /// Arguments:
    /// - path (mandatory): to the file
    /// - n1 (mandatory): the line begin
    /// - n2 (mandatory): the line end
    /// - remove_empty_string (optional): remove all string that only contains
    /// spaces
    /// - regex_keep (optional): a list of regex to keep
    /// - regex_pass (optional): a list of regex to pass
    /// - restrict (optional): if enable, it will only take the lines between 
    /// n1 and n2. If not, it will take the lines between n1 and n2 that can be
    /// taken (you can take a look at the README to have further explaination) 
    /// 
    /// return:
    /// - A list of string 
    /// 
    pub fn between(
        path: String,
        n1: usize,
        n2: usize,
        remove_empty_string: bool,
        regex_keep: Vec<String>,
        regex_pass: Vec<String>,
        restrict: bool,
    ) -> Vec<String> {
        ...
    }

    ///
    /// [WithEOL]\[tail\]: take the last n lines. \
    /// Arguments:
    /// - path (mandatory): to the file
    /// - n (mandatory): number of lines
    /// - remove_empty_string (optional): remove all string that only contains
    /// spaces
    /// - regex_keep (optional): a list of regex to keep
    /// - regex_pass (optional): a list of regex to pass
    /// - restrict (optional): if enable, it will only take the last n lines.
    /// If not, it will take the last n lines that can be taken (you can take
    /// a look at the README to have further explaination)
    /// 
    /// return:
    /// - A list of string 
    /// 
    pub fn tail(
        path: String,
        n: usize,
        remove_empty_string: bool,
        regex_keep: Vec<String>,
        regex_pass: Vec<String>,
        restrict: bool,
    ) -> Vec<String> {
        ...
    }

    ///
    /// [WithEOL]\[parse\]: take the whole file \
    /// Arguments:
    /// - path (mandatory): to the file
    /// - remove_empty_string (optional): remove all string that only contains
    /// spaces
    /// - regex_keep (optional): a list of regex to keep
    /// - regex_pass (optional): a list of regex to pass
    /// 
    /// return:
    /// - A list of string 
    /// 
    pub fn parse(
        path: String,
        remove_empty_string: bool,
        regex_keep: Vec<String>,
        regex_pass: Vec<String>,
    ) -> Vec<String> {
        ...
    }

    ///
    /// [WithEOL]\[parse\]: count the number of matched lines \
    /// Arguments:
    /// - path (mandatory): to the file
    /// - remove_empty_string (optional): remove all string that only contains
    /// spaces
    /// - regex_keep (optional): a list of regex to keep
    /// - regex_pass (optional): a list of regex to pass
    /// 
    /// return:
    /// - number of lines => usize
    ///
    pub fn count_lines(
        path: String,
        remove_empty_string: bool,
        regex_keep: Vec<String>,
        regex_pass: Vec<String>,
    ) -> usize {
        ...
    }
}
```

```rs
...
impl WithCustomDelims {
    ///
    /// [WithCustomDelims]\[head\]: take the first n lines. \
    /// Arguments:
    /// - path (mandatory): to the file
    /// - n (mandatory): number of lines
    /// - delimiter (mandatory): a list of custom delimiters
    /// - remove_empty_string (optional): remove all string that only contains
    /// spaces
    /// - regex_keep (optional): a list of regex to keep
    /// - regex_pass (optional): a list of regex to pass
    /// - restrict (optional): if enable, it will only take the first n lines.
    /// If not, it will take the first n lines that can be taken (you can take
    /// a look at the README to have further explaination)
    /// 
    /// return:
    /// - A list of string 
    /// 
    pub fn head(
        path: String,
        n: usize,
        delimiter: Vec<String>,
        remove_empty_string: bool,
        regex_keep: Vec<String>,
        regex_pass: Vec<String>,
        restrict: bool,
        buffer_size: usize,
    ) -> Vec<String> {
        ...
    }

    ///
    /// [WithCustomDelims]\[between\]: take the lines between n1 and n2 \
    /// Arguments:
    /// - path (mandatory): to the file
    /// - n1 (mandatory): the line begin
    /// - n2 (mandatory): the line end
    /// - delimiter (mandatory): a list of custom delimiters
    /// - remove_empty_string (optional): remove all string that only contains
    /// spaces
    /// - regex_keep (optional): a list of regex to keep
    /// - regex_pass (optional): a list of regex to pass
    /// - restrict (optional): if enable, it will only take the lines between 
    /// n1 and n2. If not, it will take the lines between n1 and n2 that can be
    /// taken (you can take a look at the README to have further explaination) 
    /// 
    /// return:
    /// - A list of string 
    /// 
    pub fn between(
        path: String,
        n1: usize,
        n2: usize,
        delimiter: Vec<String>,
        remove_empty_string: bool,
        regex_keep: Vec<String>,
        regex_pass: Vec<String>,
        restrict: bool,
        buffer_size: usize,
    ) -> Vec<String> {
        ...
    }

    ///
    /// [WithCustomDelims]\[tail\]: take the last n lines. \
    /// Arguments:
    /// - path (mandatory): to the file
    /// - n (mandatory): number of lines
    /// - delimiter (mandatory): a list of custom delimiters
    /// - remove_empty_string (optional): remove all string that only contains
    /// spaces
    /// - regex_keep (optional): a list of regex to keep
    /// - regex_pass (optional): a list of regex to pass
    /// - restrict (optional): if enable, it will only take the last n lines.
    /// If not, it will take the last n lines that can be taken (you can take
    /// a look at the README to have further explaination)
    /// 
    /// return:
    /// - A list of string 
    /// 
    #[staticmethod]
    #[pyo3(signature = (path, n, delimiter, remove_empty_string=false, regex_keep=Vec::new(), regex_pass=Vec::new(), restrict = true, buffer_size = 1024))]
    pub fn tail(
        path: String,
        n: usize,
        delimiter: Vec<String>,
        remove_empty_string: bool,
        regex_keep: Vec<String>,
        regex_pass: Vec<String>,
        restrict: bool,
        buffer_size: usize,
    ) -> Vec<String> {
        ...
    }

    ///
    /// [WithCustomDelims]\[parse\]: take the whole file \
    /// Arguments:
    /// - path (mandatory): to the file
    /// - delimiter (mandatory): a list of custom delimiters
    /// - remove_empty_string (optional): remove all string that only contains
    /// spaces
    /// - regex_keep (optional): a list of regex to keep
    /// - regex_pass (optional): a list of regex to pass
    /// 
    /// return:
    /// - A list of string 
    /// 
    #[staticmethod]
    #[pyo3(signature = (path, delimiter, remove_empty_string=false, regex_keep=Vec::new(), regex_pass=Vec::new(), buffer_size = 1024))]
    pub fn parse(
        path: String,
        delimiter: Vec<String>,
        remove_empty_string: bool,
        regex_keep: Vec<String>,
        regex_pass: Vec<String>,
        buffer_size: usize,
    ) -> Vec<String> {
        ...
    }

    ///
    /// [WithCustomDelims]\[parse\]: count the number of matched lines \
    /// Arguments:
    /// - path (mandatory): to the file
    /// - delimiter (mandatory): a list of custom delimiters
    /// - remove_empty_string (optional): remove all string that only contains
    /// spaces
    /// - regex_keep (optional): a list of regex to keep
    /// - regex_pass (optional): a list of regex to pass
    /// 
    /// return:
    /// - number of lines => usize
    ///
    pub fn count_lines(
        path: String,
        delimiter: Vec<String>,
        remove_empty_string: bool,
        regex_keep: Vec<String>,
        regex_pass: Vec<String>,
        buffer_size: usize,
    ) -> usize {
        ...
    }
...
}

```

## Structure

- **src/**: all sources files
- **tests/**: all tests for rust
- **tests_files/**: all files used for tests
- **tests_python/**: a python script to test