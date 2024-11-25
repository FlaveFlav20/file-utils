from_file: str = "./tests_files/DDHC.txt"
target_file: str = "./tests_files/DDHC_custom_delims.txt"

f_from = open(from_file, "r")
f_target = open(target_file, "w")

content: str = f_from.read()
res: str = ""

delims: list = ["::", ":;", "|", "éè", "小六号", "毫"]
index: int = 0

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