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