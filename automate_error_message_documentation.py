# extracting possible error lines from files

"""
file_names = ["src/main.rs", "src/tokenizer.rs", "src/execution.rs"]
keywords = ["INPUT ERROR", "INDENTATION ERROR", "BLOCK CODE ISN'T CLOSED", "INPUT INCLUDES LOWERCASE CHARACTER", "INTERPRETER ERROR", "ERROR_MESSAGE", "EXECUTION ERROR"]

for file_name in file_names:
    file = open(file_name, "r")
    lines = file.readlines()
    line_number = 0

    lines_with_keywords = []
    
    for line in lines:
        line = line.strip()
        for keyword in keywords:
            if keyword in line:
                lines_with_keywords.append(line)
        line_number += 1
    
    print("-----\nfile_name: {}\n-----".format(file_name))
    for line_with_keyword in lines_with_keywords:
        print("{}: {}\n".format(line_number, line_with_keyword))
"""

# when all valid lines are in docs/error_messages.txt delete duplicates and write to new file

file = open("docs/error_messages.txt", "r")
lines = file.readlines()

all_lines = []
for line in lines:
    all_lines.append(line)
all_lines = list(dict.fromkeys(all_lines))

# new_file = open("docs/error_messages_duplicate.txt", "w")
new_file = open("docs/error_messages.txt", "w")

for line in all_lines:
    new_file.writelines(line)
    new_file.write("\n")

new_file.close()
