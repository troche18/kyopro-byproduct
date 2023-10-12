INF = 1 << 61
from bisect import *

# 狭義


def LIS(lis):
    """
    LISをする関数
    添え字が増加列の長さで、値はのその長さの増加列の最終要素の最小値

    引数:
    lis (list): 配列

    戻り値:
    int: 増加列の長さ
    """
    n = len(lis)
    dp = [INF] * (n + 1)
    for i in lis:
        dp[bisect_left(dp, i)] = i

    return bisect_left(dp, INF)


def LIS(lis):
    """
    LISを求めて辞書順最小となるものを復元する関数
    添え字が増加列の長さで、値はのその長さの増加列の最終要素の最小値

    引数:
    lis (list): 配列

    戻り値:
    list: 復元したLIS
    """
    n = len(lis)
    dp = [INF] * (n + 1)
    idx_list = [0] * (n + 1)
    for i in range(n):
        bis = bisect_left(dp, lis[i])
        dp[bis] = lis[i]
        idx_list[i] = bis

    target_idx = max(idx_list)
    ans = [0] * (target_idx + 1)
    for i in range(n - 1, -1, -1):
        if idx_list[i] == target_idx:
            ans[target_idx] = lis[i]
            target_idx -= 1

    return ans


# 広義


def LIS_2(lis):
    """
    LISをする関数
    添え字が増加列の長さで、値はのその長さの増加列の最終要素の最小値

    引数:
    lis (list): 配列

    戻り値:
    int: 増加列の長さ
    """
    n = len(lis)
    dp = [INF] * (n + 1)
    for i in lis:
        dp[bisect_right(dp, i)] = i

    return bisect_left(dp, INF)


def LIS_2(lis):
    """
    LISを求めて辞書順最小となるものを復元する関数
    添え字が増加列の長さで、値はのその長さの増加列の最終要素の最小値

    引数:
    lis (list): 配列

    戻り値:
    list: 復元したLIS
    """
    n = len(lis)
    dp = [INF] * (n + 1)
    idx_list = [0] * (n + 1)
    for i in range(n):
        bis = bisect_right(dp, lis[i])
        dp[bis] = lis[i]
        idx_list[i] = bis

    target_idx = max(idx_list)
    ans = [0] * (target_idx + 1)
    for i in range(n - 1, -1, -1):
        if idx_list[i] == target_idx:
            ans[target_idx] = lis[i]
            target_idx -= 1

    return ans
