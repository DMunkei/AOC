def get_stairs():
    res = 0
    with open('input.txt') as f:
        while True:
            c = f.read(1)
            if not c:
                break
            if c == '(':
                res += 1
            elif c == ')':
                res -= 1
        return res


def get_stairs_2():
    count = 0
    res = 1
    with open('input.txt') as f:
        while True:
            c = f.read(1)
            if c == '(':
                count += 1
            elif c == ')':
                count -= 1
            if count < 0:
                return res
            res += 1


print(get_stairs())
print(get_stairs_2())
