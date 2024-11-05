import file_operations_lib
import unittest
import os

# Variables
from custom_files import path
# Functions
from custom_files import get_list

class TestWithEOFParse(unittest.TestCase):
    def test_parse_no_options(self):
        file = os.popen("cat " + path)
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.parse(path=path), ref)

    def test_parse_remove_empty_string(self):
        file = os.popen("sed '/^$/d' " + path)
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.parse(path=path, remove_empty_string=True), ref)

    def test_parse_keep_when_regex(self):
        file = os.popen("grep \"^La loi\" " + path)
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.parse(path=path, remove_empty_string=False, regex_keep=["^La loi.*"]), ref)

    def test_parse_pass_when_regex(self):
        file = os.popen("grep -v \"^La loi\" " + path)
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.parse(path=path, remove_empty_string=False, regex_pass=["^La loi.*"]), ref)
    
    def test_remove_empty_string_parse_keep_when_regex(self):
        file = os.popen("sed '/^$/d' " + path + " | grep \"^La loi\" ")
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.parse(path=path, remove_empty_string=True, regex_keep=["^La loi.*"]), ref)

    def test_parse_pass_when_regex(self):
        file = os.popen("sed '/^$/d' " + path + " | grep -v \"^La loi\" ")
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.parse(path=path, remove_empty_string=True, regex_pass=["^La loi.*"]), ref)

if __name__ == '__main__':
    unittest.main()