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

    cwd = os.getcwd()
    os.chdir(os.path.dirname(os.path.realpath(__file__)))

    from WithCustomDelims_OEL.tests_tail import TestWithCustomDelimEOFTail
    from WithCustomDelims_OEL.tests_head import TestWithCustomDelimEOFHead
    from WithCustomDelims_OEL.tests_between import TestWithCustomDelimEOFBetween
    from WithCustomDelims_OEL.tests_parse import TestWithCustomDelimEOFParse
    from WithCustomDelims_OEL.tests_count_lines import TestWithCustomDelimEOFCountLines

    from WithCustomDelims.tests_tail import TestWithCustomDelimTail
    from WithCustomDelims.tests_head import TestWithCustomDelimHead
    from WithCustomDelims.tests_between import TestWithCustomDelimBetween
    from WithCustomDelims.tests_parse import TestWithCustomDelimParse
    from WithCustomDelims.tests_count_lines import TestWithCustomDelimCountLines
    os.chdir(cwd)
