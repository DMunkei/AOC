import util
import re
from collections import namedtuple
from functools import reduce

Colour = namedtuple('Colour', ['colour', 'limit', 'regex'])
colours = [Colour(colour, limit, re.compile(rf'\d+ {colour}')) for (colour, limit) in [('red',12), ('green', 13), ('blue', 14)]]

def solve_part_1():
    def extract_highest_number_from(colour, line):
        colour_match = colour.regex.findall(line)
        return max(map(int, "".join(colour_match).replace(colour.colour, "").strip().split(" ")))
    res = 0
    for idx, line in enumerate(util.get_real_data(2, 1), start=1):
        can_sum = True
        round_product = []
        for colour in colours:
            max_digit = extract_highest_number_from(colour, line)
            round_product.append(max_digit)
            if max_digit > colour.limit:
                can_sum = False
                break
        if can_sum:
            res += idx
    print(res)


def solve_part_2():
    def extract_highest_number_from(colour, line):
        colour_match = colour.regex.findall(line)
        return max(map(int, "".join(colour_match).replace(colour.colour, "").strip().split(" ")))
    res = 0
    for line in util.get_real_data(2, 2):
        round_product = []
        for colour in colours:
            round_product.append(extract_highest_number_from(colour, line))
        power_set = reduce(lambda x,y: x*y, round_product)
        res += power_set
    print(res)

# solve_part_1()
solve_part_2()
