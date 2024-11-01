import file_operations_lib
import unittest
import os

path: str = "./tests_files/DDHC.txt"

def get_list(string: str) -> list:
    res: list = string.split("\n")
    if len(res) == 1 and res[0] == '':
        return []
    return res 

class TestWithEOFTail(unittest.TestCase):
    def test_tail_n_10_valid_remove_empty_string_false(self):
        n: int = 10
        file = os.popen("tail " + path + " -n " + str(n))
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.tail(file=path, n=n), ref)

    def test_tail_n_0_valid_remove_empty_string_false(self):
        n: int = 0
        file = os.popen("tail " + path + " -n " + str(n))
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.tail(file=path, n=n), ref)

    def test_tail_n_1_valid_remove_empty_string_false(self):
        n: int = 1
        file = os.popen("tail " + path + " -n " + str(n))
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.tail(file=path, n=n), ref)

    def test_tail_n_neg_invalid_remove_empty_string_false(self):
        check_pass: bool = True 
        try:
            file_operations_lib.WithEOL.tail(file=path, n=n)
        except:
            check_pass = False
        if check_pass:
            self.fail("[tail] Negative value shouldn't work")

##

    def test_tail_n_10_valid_remove_empty_string_true(self):
        n: int = 10
        file = os.popen("sed '/^$/d' " + path + " | tail " + " -n " + str(n))
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.tail(file=path, n=n, remove_empty_string=True), ref)

    def test_tail_n_0_valid_remove_empty_string_true(self):
        n: int = 0
        file = os.popen("sed '/^$/d' " + path + " | tail " + " -n " + str(n))
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.tail(file=path, n=n, remove_empty_string=True), ref)

    def test_tail_n_1_valid_remove_empty_string_true(self):
        n: int = 1
        file = os.popen("sed '/^$/d' " + path + " | tail " + " -n " + str(n))
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.tail(file=path, n=n, remove_empty_string=True), ref)

    def test_tail_n_neg_invalid_remove_empty_string_true(self):
        check_pass: bool = True 
        try:
            file_operations_lib.WithEOL.tail(file=path, n=n, remove_empty_string=True)
        except:
            check_pass = False
        if check_pass:
            self.fail("[tail] Negative value shouldn't work")