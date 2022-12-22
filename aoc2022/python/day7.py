from collections import defaultdict


def main():
    with open("7.prod") as f:
        cwd = []
        dirsizes = defaultdict(int)
        for line in f:
            tokens = line.strip().split(' ')
            if tokens[0] == '$' and tokens[1] == 'cd':
                # eject one directory up
                if tokens[2] == '..':
                    cwd.pop()
                # reset path
                elif tokens[2] == '/':
                    cwd = ['/']
                # add onto path
                else:
                    cwd.append(tokens[2])
            elif tokens[0] == "dir" or tokens[0] == '$' and tokens[1] == 'ls':
                pass
            else:
                size = int(tokens[0])
                for i in range(0, len(cwd)):
                    dirsizes['/'.join(cwd[:i + 1])] += size

    part1 = sum(s for s in dirsizes.values() if s <= 100000)
    print(f'part1= {part1}')
    total = 70_00_00_00
    used_space = dirsizes['/']
    free = total - used_space
    target = 30_00_00_00
    candidates = []
    for size in dirsizes.values():
        if free + size >= target:
            candidates.append(size)
    print(f'{candidates}')
    part2 = min(candidates)
    print(f'part2 = {part2}')


main()
