import unittest

import os

from WithEOF.tests_tail import TestWithEOFTail
from WithEOF.tests_head import TestWithEOFHead
from WithEOF.tests_between import TestWithEOFBetween
from WithEOF.tests_parse import TestWithEOFParse
from WithEOF.tests_count_lines import TestWithEOFCountLines

from custom_files import create_regex_test_file, custom_path

create_regex_test_file(custom_path)

if __name__ == '__main__':
    unittest.main()