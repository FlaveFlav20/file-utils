import os
from pathlib import Path

custom_path: str = os.path.join(Path(os.path.realpath(__file__)).parent, "..", "tests_files", "custom.txt")
custom_path_delim: str = os.path.join(Path(os.path.realpath(__file__)).parent, "..", "tests_files", "custom_delim.txt")
path: str = os.path.join(Path(os.path.realpath(__file__)).parent, "..", "tests_files", "DDHC.txt")
path_delim: str = os.path.join(Path(os.path.realpath(__file__)).parent, "..", "tests_files", "DDHC_custom_delims.txt")
custom_path_delim: str = os.path.join(Path(os.path.realpath(__file__)).parent, "..", "tests_files", "custom_delim.txt")
delimiter: list = ["::", ":;", "|", "éè", "小六号", "毫"]

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

def create_delim_test_file(ref_path: str, to_path: str):
    delims: list = ["::", ":;", "|", "éè", "小六号", "毫"]
    index: int = 0
    try:
        f_from = open(ref_path, "r")
        f_target = open(to_path, "w")

        content: str = f_from.read()
        res: str = ""

        for c in content:
            if c == '\n':
                res += delims[index]
                index += 1
                index = index % len(delims)
            else:
                res += c

        f_target.write(res)

        f_target.close()
        f_from.close()
    except:
        return

# get_list: string to list converter
#   - args: a string -> output of a command
#   - return: a list that contain each line of the output

def get_list(string: str) -> list:
    res: list = string.split("\n")
    if len(res) == 1 and res[0] == '':
        return []
    elif len(res) > 1 and res[-1] == '':
        res.pop()
    return res 