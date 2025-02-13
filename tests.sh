#!/bin/bash

### Rust tests
python3.12 tests_files/create_test_custom_delims_file.py;
if test $? -ne 0; then
    echo "Unable to create the rust test file";
    exit 1;
fi;

cargo test;
if test $? -ne 0; then
    echo "Rust code has a problem";
    exit 1;
fi

###

### Python tests
# Build the env
python3.12 -m venv env;
if test $? -ne 0; then
    echo "Unable to build the python env";
    exit 1;
fi


source env/bin/activate;
if test $? -ne 0; then
    echo "Unable to activate the python env";
    exit 1;
fi
#

# Install dependencies
pip install maturin;
if test $? -ne 0; then
    echo "Unable to install maturin";
    exit 1;
fi

maturin develop;
if test $? -ne 0; then
    echo "Unable to delelop the library";
    exit 1;
fi

# Create the test file
python3.12 tests_python/custom_files.py;
if test $? -ne 0; then
    echo "Unable to create the python test file";
    exit 1;
fi
#

cd tests_python/;
if test $? -ne 0; then
    echo "tests_python folder doesn't exist";
    exit 1;
fi

# Pythons tests
python tests.py;
if test $? -ne 0; then
    echo "error while testing python library";
    cd ..;
    exit 1;
fi
#

cd ..;
exit 0;

###
