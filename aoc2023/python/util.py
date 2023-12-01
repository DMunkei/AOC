def get_test_data(part: int = 1):
    with open(f'../data/data.test_{part}') as f:
        for line in f.readlines():
            yield line.strip()


def get_real_data(level: int, part: int):
    with open(f'../data/{level}_{part}.input') as f:
        for line in f.readlines():
            yield line.strip()

def _hidden():
    print("I'm hidden!")

if (__name__ == '__main__'):
    get_test_data(1)
