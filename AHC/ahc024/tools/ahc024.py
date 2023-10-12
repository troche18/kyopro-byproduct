# import pypyjit;pypyjit.set_param("max_unroll_recursion=-1")
# from bisect import *
# from collections import deque
# from heapq import *
# from itertools import *
# from sortedcontainers import *
# from math import gcd, lcm,dist
# from datetime import *
# from decimal import *  # PyPyだと遅い
# from string import ascii_lowercase,ascii_uppercase
# import numpy as np
from atcoder.dsu import *

# from atcoder.segtree import *
# from sortedcontainers import *
from random import shuffle, seed
import sys
import os

is_atcoder = os.getenv("ATCODER", 0)
# sys.setrecursionlimit(10**6) # PyPyは呪文を付ける
INF = 1 << 61
MOD = 998244353
# MOD = 10**9 + 7
File = sys.stdin


def input():
    return File.readline()[:-1]


# ///////////////////////////////////////////////////////////////////////////
dir4 = [[1, 0], [0, 1], [-1, 0], [0, -1]]
seed(24)


def search(v, points_set, grid, N):
    """
    隣接している頂点を列挙する
    連結しているかも判定
    """
    near_set = set()
    dict_num = {}
    for i, j in enumerate(points_set[v]):
        dict_num[j] = i
    uf = DSU(len(points_set[v]))
    for i, j in points_set[v]:
        for di, dj in dir4:
            if i + di > N or j + dj > N:
                continue
            if (i + di, j + dj) not in points_set[v]:
                near_set.add(grid[i + di][j + dj])
            else:
                uf.merge(dict_num[(i + di, j + dj)], dict_num[(i, j)])
    if len(uf.groups()) != 1:
        return set()
    return near_set


def solve(N, M, grid):
    # 隣接情報をまとめる
    G = [set() for _ in range(M + 1)]
    points_set = [set() for _ in range(M + 1)]
    for i in range(N + 1):
        for j in range(N + 1):
            for di, dj in dir4:
                if i + di > N or j + dj > N:
                    continue
                if grid[i][j] != grid[i + di][j + dj]:
                    G[grid[i][j]].add(grid[i + di][j + dj])
                    G[grid[i + di][j + dj]].add(grid[i][j])
            points_set[grid[i][j]].add((i, j))

    dist = [
        sum([max(abs(i - 25), abs(j - 25)) for i, j in points_set[k]])
        / len(points_set[k])
        for k in range(M + 1)
    ]
    sorted_points = list(
        sorted(enumerate(dist), key=lambda x: (x[1], -len(points_set[x[0]])))
    )
    idx = 0
    for _ in range(5):
        seen = set()
        for v, _ in sorted_points:
            if v == 0:
                continue
            seen.add(v)
            lis = list(points_set[v])
            shuffle(lis)
            while 1:
                if len(points_set[v]) <= 1:
                    break
                b = len(points_set[v])
                for i, j in lis:
                    if len(points_set[v]) <= 1:
                        break
                    if (i, j) not in points_set[v]:
                        continue
                    for di, dj in dir4:
                        if (i + di, j + dj) not in points_set[v] and grid[i + di][
                            j + dj
                        ] not in seen:
                            grid[i][j] = grid[i + di][j + dj]
                            points_set[v].discard((i, j))
                            points_set[grid[i + di][j + dj]].add((i, j))
                            target = set()
                            for di2, dj2 in dir4:
                                target.add(grid[i + di2][j + dj2])
                                target.add(grid[di + di2][dj + dj2])
                            for near in target:
                                if near == 0:
                                    continue
                                if G[near] != search(near, points_set, grid, N):
                                    break
                            else:
                                break
                            grid[i][j] = v
                            points_set[v].add((i, j))
                            points_set[grid[i + di][j + dj]].discard((i, j))

                a = len(points_set[v])
                if b == a:
                    break
            idx += 1
            if idx % 3 == 0:
                for i in grid[:-1]:
                    print(*i[:-1])

    for i in grid[:-1]:
        print(*i[:-1])


if __name__ == "__main__":
    if is_atcoder:
        solve(
            *map(int, input().split()),
            [list(map(int, input().split())) + [0] for _ in range(50)] + [[0] * (51)]
        )
    else:
        solve(
            *map(int, input().split()),
            [list(map(int, input().split())) + [0] for _ in range(50)] + [[0] * (51)]
        )
