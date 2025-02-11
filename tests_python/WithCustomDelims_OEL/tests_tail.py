import file_utils_lib
import unittest
import os

# Variables
from custom_files import headers, path, custom_path
# Functions
from custom_files import create_regex_test_file, get_list

class TestWithCustomDelimEOFTail(unittest.TestCase):
    def test_tail_n_10_valid_remove_empty_string_false(self):
        n: int = 10
        file = os.popen("tail " + path + " -n " + str(n))
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_utils_lib.WithCustomDelims.tail(path=path, n=n, delimiter=['\n']), ref)

    def test_tail_n_0_valid_remove_empty_string_false(self):
        n: int = 0
        file = os.popen("tail " + path + " -n " + str(n))
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_utils_lib.WithCustomDelims.tail(path=path, n=n, delimiter=['\n']), ref)

    def test_tail_n_1_valid_remove_empty_string_false(self):
        n: int = 1
        file = os.popen("tail " + path + " -n " + str(n))
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_utils_lib.WithCustomDelims.tail(path=path, n=n, delimiter=['\n']), ref)

    def test_tail_n_neg_invalid_remove_empty_string_false(self):
        check_pass: bool = True 
        try:
            file_utils_lib.WithCustomDelims.tail(path=path, n=n)
        except:
            check_pass = False
        if check_pass:
            self.fail("[tail] Negative value shouldn't work")

## 

    def test_custom_path_tail_n_10_valid_remove_empty_string_false(self):
        n: int = 10
        file = os.popen("tail " + custom_path + " -n " + str(n))
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_utils_lib.WithCustomDelims.tail(path=custom_path, n=n, delimiter=['\n']), ref)

    def test_custom_path_tail_n_0_valid_remove_empty_string_false(self):
        n: int = 0
        file = os.popen("tail " + custom_path + " -n " + str(n))
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_utils_lib.WithCustomDelims.tail(path=path, n=n, delimiter=['\n']), ref)

    def test_custom_path_tail_n_1_valid_remove_empty_string_false(self):
        n: int = 1
        file = os.popen("tail " + custom_path + " -n " + str(n))
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_utils_lib.WithCustomDelims.tail(path=custom_path, n=n, delimiter=['\n']), ref)

    def test_custom_path_tail_n_neg_invalid_remove_empty_string_false(self):
        check_pass: bool = True 
        try:
            file_utils_lib.WithCustomDelims.tail(path=path, n=n)
        except:
            check_pass = False
        if check_pass:
            self.fail("[tail] Negative value shouldn't work")

## Test empty lines ----------

    def test_tail_n_10_valid_remove_empty_string_true(self):
        n: int = 10
        file = os.popen("sed '/^$/d' " + path + " | tail " + " -n " + str(n))
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_utils_lib.WithCustomDelims.tail(path=path, n=n, delimiter=['\n'], remove_empty_string=True, restrict=False), ref)

    def test_tail_n_0_valid_remove_empty_string_true(self):
        n: int = 0
        file = os.popen("sed '/^$/d' " + path + " | tail " + " -n " + str(n))
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_utils_lib.WithCustomDelims.tail(path=path, n=n, delimiter=['\n'], remove_empty_string=True), ref)

    def test_tail_n_1_valid_remove_empty_string_true(self):
        n: int = 1
        file = os.popen("sed '/^$/d' " + path + " | tail " + " -n " + str(n))
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_utils_lib.WithCustomDelims.tail(path=path, n=n, delimiter=['\n'], remove_empty_string=True), ref)

    def test_tail_n_neg_invalid_remove_empty_string_true(self):
        check_pass: bool = True 
        try:
            file_utils_lib.WithCustomDelims.tail(path=path, n=n, delimiter=['\n'], remove_empty_string=True)
        except:
            check_pass = False
        if check_pass:
            self.fail("[tail] Negative value shouldn't work")

## 

    def test_custom_path_tail_n_10_valid_remove_empty_string_true(self):
        n: int = 10
        file = os.popen("sed '/^$/d' " + custom_path + " | tail " + " -n " + str(n))
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_utils_lib.WithCustomDelims.tail(path=custom_path, n=n, delimiter=['\n'], remove_empty_string=True), ref)

    def test_custom_path_tail_n_0_valid_remove_empty_string_true(self):
        n: int = 0
        file = os.popen("sed '/^$/d' " + custom_path + " | tail " + " -n " + str(n))
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_utils_lib.WithCustomDelims.tail(path=path, n=n, delimiter=['\n'], remove_empty_string=True), ref)

    def test_custom_path_tail_n_1_valid_remove_empty_string_true(self):
        n: int = 1
        file = os.popen("sed '/^$/d' " + custom_path + " | tail " + " -n " + str(n))
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_utils_lib.WithCustomDelims.tail(path=custom_path, n=n, delimiter=['\n'], remove_empty_string=True), ref)

    def test_custom_path_tail_n_neg_invalid_remove_empty_string_true(self):
        check_pass: bool = True 
        try:
            file_utils_lib.WithCustomDelims.tail(path=path, n=n, delimiter=['\n'], remove_empty_string=True)
        except:
            check_pass = False
        if check_pass:
            self.fail("[tail] Negative value shouldn't work")

## Test regex

    def test_tail_n_valid_invalid_remove_empty_string_false_keep_when_regex_invalid(self):
        check_pass: bool = True 
        try:
            file_utils_lib.WithCustomDelims.tail(path=path, n=n, delimiter=['\n'], remove_empty_string=True, regex_keep="Haha")
        except:
            check_pass = False
        if check_pass:
            self.fail("[tail] Non bool value shouldn't work")

    def test_tail_n_valid_invalid_remove_empty_string_false_keep_when_regex_valid_pass_when_regex_invalid(self):
        check_pass: bool = True 
        try:
            file_utils_lib.WithCustomDelims.tail(path=path, n=n, delimiter=['\n'], remove_empty_string=True, regex_pass="Haha")
        except:
            check_pass = False
        if check_pass:
            self.fail("[tail] Non bool value shouldn't work")
        
    def test_tail_n_valid_invalid_remove_empty_string_false_keep_when_regex_valid_pass_when_regex_valid_regex_invalid(self):
        check_pass: bool = True 
        try:
            file_utils_lib.WithCustomDelims.tail(path=path, n=n, delimiter=['\n'], remove_empty_string=True, regex=1)
        except:
            check_pass = False
        if check_pass:
            self.fail("[tail] Non bool value shouldn't work")

    def test_tail_n_10_invalid_remove_empty_string_false_keep_when_regex_valid_pass_when_regex_valid_regex_Warning(self):
        n: int = 10
        global headers
        result_to_test: list = file_utils_lib.WithCustomDelims.tail(path=custom_path, n=n, delimiter=['\n'], regex_keep=["\[Warning\]:.*"], restrict=True)

        self.assertEqual(len(result_to_test), n // len(headers) + 1)

    def test_tail_n_10_invalid_remove_empty_string_false_keep_when_regex_valid_pass_when_regex_valid_regex_Info(self):
        n: int = 10
        global headers
        result_to_test: list = file_utils_lib.WithCustomDelims.tail(path=custom_path, n=n, delimiter=['\n'], regex_keep=["\[Info\]:.*"], restrict=True)

        self.assertEqual(len(result_to_test), n // len(headers) + 1)

    def test_tail_n_10_invalid_remove_empty_string_false_keep_when_regex_valid_pass_when_regex_valid_regex_Error(self):
        n: int = 10
        global headers
        result_to_test: list = file_utils_lib.WithCustomDelims.tail(path=custom_path, n=n, delimiter=['\n'], regex_keep=["\[Error\]:.*"], restrict=True)

        self.assertEqual(len(result_to_test), n // len(headers))



if __name__ == '__main__':
    create_regex_test_file(custom_path)
    unittest.main()
