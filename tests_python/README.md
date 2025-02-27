# file utils: Python tests

## Intro

These tests have been tested on Linux (ubutu)

## How to run tests?

To run tests, you should execute these commands:

1) You must create your environment 
```sh
python3 -m venv env && source env/bin/activate && pip install maturin && maturin develop
```
2) You must create test files
```sh
python tests_files/create_test_custom_delims_file.py
cd tests_python/ && python custom_files.py && cd ..
```
3) Then, you should run tests with
```sh
 cd tests_python/ && python tests_python/tests.py && cd ..
```

## How to run a single test?

At the 3 step, you can go to an specific test like **tests_python/WithCustomDelims** with
```sh
cd tests_python/WithCustomDelims
``
and run a test like
```sh
python tests_between.py
```
Don't forget to 

```sh
python tests_files/create_test_custom_delims_file.py
cd tests_python/ && python custom_files.py && cd ..
```
You should use an environement.

## Tests format
```sh
tree tests_python
```
```sh
.
├── custom_files.py                 # To create the custom file for all tests
├── README.md                       # This file
├── tests.py                        # Run all tests
├── WithCustomDelims                # Folder name => The method that is tested
│   ├── custom_files.py             # To create the custom file for a specific test
│   ├── tests_between.py            # The function that has been tested
│   ├── tests_count_lines.py        #...
│   ├── tests_head.py
│   ├── tests_parse.py
│   └── tests_tail.py
├── WithCustomDelims_OEL
│   ├── custom_files.py
│   ...
└── WithEOF
    ├─...
    ...
```

The **_EOL** is just a custom_delim test with the delimiter "**\n**"
