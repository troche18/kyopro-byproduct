class BIT:
    def __init__(self, size):
        self.size = size
        self.tree = [0] * (size + 1)

    def update(self, index, val):
        while index <= self.size:
            self.tree[index] += val
            index += index & -index

    def query(self, index):
        total = 0
        while index > 0:
            total += self.tree[index]
            index -= index & -index
        return total


def count_inversions(arr):
    """
    転倒数をBITを使用して数える関数

    引数
    arr (list): 転倒数を数えるリスト

    戻り値
    int: 転倒数
    """
    n = len(arr)
    sorted_arr = sorted(arr)
    rank_map = {value: idx + 1 for idx, value in enumerate(sorted_arr)}
    inversion_count = 0
    bit = BIT(n)

    for value in reversed(arr):
        rank = rank_map[value]
        inversion_count += bit.query(rank - 1)
        bit.update(rank, 1)

    return inversion_count
