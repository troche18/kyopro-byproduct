# import pypyjit;pypyjit.set_param("max_unroll_recursion=-1")
# from bisect import *
# from collections import *
# from heapq import *
# from itertools import *
# from sortedcontainers import *
# from math import gcd, lcm
# from datetime import *
# from decimal import *  # PyPyだと遅い
# from string import ascii_lowercase,ascii_uppercase
# import numpy as np
# from atcoder.dsu import *
# from atcoder.segtree import *
# from sortedcontainers import *
# from random import *
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


N = int(input())
