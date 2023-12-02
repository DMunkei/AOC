import util

def find_digit(line, rev = False):
    word_mapping = {
        "one":"1",
        "two":"2",
        "three":"3",
        "four":"4",
        "five":"5",
        "six":"6",
        "seven":"7",
        "eight":"8",
        "nine":"9",
    }
    for idx, c in enumerate(line):
        if str.isdigit(c):
            return c
        else:
            for number in word_mapping.keys():
                if len(number) + idx < len(line):
                    rs = line[idx:idx+len(number)]
                    if rev:
                        rs = rs[::-1]
                    if rs in word_mapping:
                        return word_mapping.get(rs)

def solve_part_2():
    res = 0
    for line in util.get_test_data(2):
    # for line in util.get_real_data(1,2):
        first_digit = find_digit(line)
        rev_line = line[::-1]
        second_digit = find_digit(rev_line, True)
        res += int(f"{first_digit}{second_digit}")
        first_digit = 0
        second_digit = 0
    return res

def solve_part_2v2():
    numbers = [ "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" ]
    res = 0
    for line in util.get_test_data(2):
        digits = []
        for idx, c in enumerate(line):
            if c.isdigit():
                digits.append(c)
            for dig, number in enumerate(numbers):
                if line[idx:].startswith(number):
                    digits.append(str(dig+1))
        res += int(f"{digits[0]}{digits[-1]}") 
    return res



def solve_part_1():
    res = 0
    for line in util.get_test_data():
        digits = "".join(filter(lambda x: str.isdigit(x), line))
        res += int(f"{digits[0]}{digits[-1]}") 
    print(res)


print(solve_part_2())
print(solve_part_2v2())
# print(foo() - 53855)

