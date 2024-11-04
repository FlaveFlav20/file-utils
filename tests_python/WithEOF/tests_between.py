import file_operations_lib
import unittest
import os

path: str = "./tests_files/DDHC.txt"

def get_list(string: str) -> list:
    res: list = string.split("\n")
    if len(res) == 1 and res[0] == '':
        return []
    elif len(res) > 1 and res[-1] == '':
        res.pop()
    return res 

class TestWithEOFBetween(unittest.TestCase):
    def test_between_n1_1_n2_1_valid_remove_empty_string_false(self):
        n1: int = 1
        n2: int = 1
        file = os.popen("sed -n '"+ str(n1) + "," + str(n2) + " p' " + path)
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.between(file=path, n1=n1, n2=n2, restrict=False), ref)

    def test_between_n1_2_n2_1_valid_remove_empty_string_false(self):
        n1: int = 2
        n2: int = 1
        res: str= ""
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.between(file=path, n1=n1, n2=n2, restrict=False), ref)

    def test_between_n1_1_n2_2_valid_remove_empty_string_false(self):
        n1: int = 1
        n2: int = 2
        file = os.popen("sed -n '"+ str(n1) + "," + str(n2) + " p' " + path)
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.between(file=path, n1=n1, n2=n2, restrict=False), ref)

    def test_between_n1_neg_n2_pos_invalid_remove_empty_string_false(self):
        check_pass: bool = True 
        n1: int = -1
        n2: int = 1
        try:
            file_operations_lib.WithEOL.between(file=path, n1=n1, n2=n2, restrict=False)
        except:
            check_pass = False
        if check_pass:
            self.fail("[between] Negative value shouldn't work")

    def test_between_n1_pos_n2_neg_invalid_remove_empty_string_false(self):
        check_pass: bool = True 
        n1: int = 1
        n2: int = -1
        try:
            file_operations_lib.WithEOL.between(file=path, n1=n1, n2=n2, restrict=False)
        except:
            check_pass = False
        if check_pass:
            self.fail("[between] Negative value shouldn't work")

    def test_between_n1_neg_n2_neg_invalid_remove_empty_string_false(self):
        check_pass: bool = True 
        n1: int = -1
        n2: int = -1
        try:
            file_operations_lib.WithEOL.between(file=path, n1=n1, n2=n2, restrict=False)
        except:
            check_pass = False
        if check_pass:
            self.fail("[between] Negative value shouldn't work")

##

    def test_between_n1_1_n2_1_valid_remove_empty_string_true(self):
        n1: int = 1
        n2: int = 1
        file = os.popen("sed '/^$/d' " + path + " | sed -n '"+ str(n1) + "," + str(n2) + " p'")
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.between(file=path, n1=n1, n2=n2, remove_empty_string=True, restrict=False), ref)

    def test_between_n1_2_n2_1_valid_remove_empty_string_true(self):
        n1: int = 2
        n2: int = 1
        res: str= ""
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.between(file=path, n1=n1, n2=n2, remove_empty_string=True, restrict=False), ref)

    def test_between_n1_1_n2_2_valid_remove_empty_string_true(self):
        n1: int = 1
        n2: int = 2
        file = os.popen("sed '/^$/d' " + path + " | sed -n '"+ str(n1) + "," + str(n2) + " p'")
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.between(file=path, n1=n1, n2=n2, remove_empty_string=True, restrict=False), ref)

    def test_between_n1_neg_n2_pos_invalid_remove_empty_string_true(self):
        check_pass: bool = True 
        n1: int = -1
        n2: int = 1
        try:
            file_operations_lib.WithEOL.between(file=path, n1=n1, n2=n2, remove_empty_string=True, restrict=False)
        except:
            check_pass = False
        if check_pass:
            self.fail("[between] Negative value shouldn't work")

    def test_between_n1_pos_n2_neg_invalid_remove_empty_string_true(self):
        check_pass: bool = True 
        n1: int = 1
        n2: int = -1
        try:
            file_operations_lib.WithEOL.between(file=path, n1=n1, n2=n2, remove_empty_string=True, restrict=False)
        except:
            check_pass = False
        if check_pass:
            self.fail("[between] Negative value shouldn't work")

    def test_between_n1_neg_n2_neg_invalid_remove_empty_string_true(self):
        check_pass: bool = True 
        n1: int = -1
        n2: int = -1
        try:
            file_operations_lib.WithEOL.between(file=path, n1=n1, n2=n2, remove_empty_string=True, restrict=False)
        except:
            check_pass = False
        if check_pass:
            self.fail("[between] Negative value shouldn't work")

##

if __name__ == '__main__':
    unittest.main()