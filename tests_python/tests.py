import unittest

import sys
import os
from os.path import dirname

sys.path.append(os.path.abspath("WithEOF"))
print(sys.path)

from WithEOF.tests_tail import TestWithEOFTail
from WithEOF.tests_head import TestWithEOFHead
from WithEOF.tests_between import TestWithEOFBetween
from WithEOF.tests_parse import TestWithEOFParse
from WithEOF.tests_count_lines import TestWithEOFCountLines

if __name__ == '__main__':
    unittest.main()