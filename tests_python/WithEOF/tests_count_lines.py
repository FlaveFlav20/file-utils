import file_operations_lib
import unittest
import os

path: str = "./tests_files/DDHC.txt"
custom_path: str = "./tests_files/custom.txt"

warning: str = "[Warning]:"
error: str = "[Error]:"
info: str = "[Info]:"

message_1: str = "Entity not found\n"
message_2: str = "Function not found\n"
message_2: str = "Unable to recover data\n"
message_3: str = "Segfault\n"
message_4: str = "Indentation\n"
message_5: str = "Memory leaks\n"
headers: list = [warning, error, info]
messages: list = [message_1, message_2, message_3, message_4, message_5]

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
        self.assertEqual(file_operations_lib.WithEOL.count_lines(file=path), len(ref))
    
    def test_remove_empty_lines(self):
        file = os.popen("sed '/^$/d' " + path)
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.count_lines(file=path, remove_empty_string=True), len(ref))
    
    def test_keep_regex(self):
        file = os.popen("grep \"^La loi\" " + path)
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.count_lines(file=path, remove_empty_string=False, keep_when_regex=True, regex="^La loi"), len(ref))
    
    def test_remove_empty_lines_keep_regex(self):
        file = os.popen("sed '/^$/d' " + path + " | grep \"^La loi\" ")
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.count_lines(file=path, remove_empty_string=True, keep_when_regex=True, regex="^La loi"), len(ref))
    
    def test_pass_regex(self):
        file = os.popen("grep -v \"^La loi\" " + path)
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.count_lines(file=path, remove_empty_string=False, pass_when_regex=True, regex="^La loi"), len(ref))

    def test_remove_empty_lines_pass_regex(self):
        file = os.popen("sed '/^$/d' " + path + " | grep -v \"^La loi\" ")
        res: str= file.read()
        file.close()
        ref: list = get_list(res)
        self.assertEqual(file_operations_lib.WithEOL.count_lines(file=path, remove_empty_string=True, pass_when_regex=True, regex="^La loi"), len(ref))

if __name__ == '__main__':
    unittest.main()