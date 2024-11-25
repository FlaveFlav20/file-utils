import sys
from pathlib import Path

sys.path.append(str(Path(__file__).parent.parent.parent))

import os
from pathlib import Path

custom_path: str = os.path.join(Path(os.path.realpath(__file__)).parent, "..", "..", "tests_files", "custom.txt")
path: str = os.path.join(Path(os.path.realpath(__file__)).parent, "..", "..", "tests_files", "DDHC.txt")

# Variables
from tests_python.custom_files import messages, headers
# Functions
from tests_python.custom_files import create_regex_test_file, get_list