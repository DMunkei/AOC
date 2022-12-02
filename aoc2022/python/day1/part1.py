import functools
def main():
    sum = 0
    max = 0

    with open('input.txt') as f:
        arr = [[]]
        for x in f:
            if x.strip() == '':
                arr.append([])
            else:
                arr[len(arr)-1].append(int(x.strip()))

        print(len(arr))

    # with open('input.txt') as f:
    #     for line in f:
    #         if line.strip() == '':
    #             if sum > max:
    #                 max = sum
    #             sum = 0
    #         else:
    #             sum += int(line)
    # print(max)


main()
