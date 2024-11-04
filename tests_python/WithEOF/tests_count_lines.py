import file_operations_lib
import unittest
import os

path: str = "./tests_files/DDHC.txt"
custom_path: str = "./tests_files/custom.txt"

def create_regex_test_file(path: str) -> bool:
    global headers
    global messages

    try:
        f = open(path, "w")

        for i in range(100):
            f.write(headers[i % len(headers)] + messages[i % len(messages)])

        f.close()
        return True
    except:
        return False

if __name__ == '__main__':
    create_regex_test_file(custom_path)

def get_list(string: str) -> list:
    res: list = string.split("\n")
    if len(res) == 1 and res[0] == '':
        return []
    elif len(res) > 1 and res[-1] == '':
        res.pop()
    return res 

class TestWithEOFCountLines(unittest.TestCase):
    def test_basic(self):
        file = os.popen("cat " + path)
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.count_lines(path=path), len(ref))
    
    def test_remove_empty_lines(self):
        file = os.popen("sed '/^$/d' " + path)
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.count_lines(path=path, remove_empty_string=True), len(ref))
    
    def test_keep_regex(self):
        file = os.popen("grep \"^La loi\" " + path)
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.count_lines(path=path, remove_empty_string=False, regex_keep=["^La loi"]), len(ref))
    
    def test_remove_empty_lines_keep_regex(self):
        file = os.popen("sed '/^$/d' " + path + " | grep \"^La loi\" ")
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.count_lines(path=path, remove_empty_string=True, regex_keep=["^La loi"]), len(ref))
    
    def test_pass_regex(self):
        file = os.popen("grep -v \"^La loi\" " + path)
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.count_lines(path=path, remove_empty_string=False, regex_pass=["^La loi"]), len(ref))

    def test_remove_empty_lines_pass_regex(self):
        file = os.popen("sed '/^$/d' " + path + " | grep -v \"^La loi\" ")
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.count_lines(path=path, remove_empty_string=True, regex_pass=["^La loi"]), len(ref))

if __name__ == '__main__':
    unittest.main()