INF = 1 << 61


def subset_sum(N, M, A, MOD):
    """
    要素数Nの整数列Aから、総和がMとなるように整数を選ぶ方法が
    何通りあるかを数え上げる

    引数:
    N (int): 要素数
    M (int): 数える数
    A (int): 数列
    MOD (int): 0なら余りを取らない

    戻り値:
    dp (list): DPテーブル dp[-1][M]が答え
    """
    dp = [[0] * (M + 1) for _ in range(N + 1)]
    dp[0][0] = 1
    for i in range(N):
        for j in range(M + 1):
            if dp[i][j]:
                dp[i + 1][j] = dp[i + 1][j] + dp[i][j]
                if MOD:
                    dp[i + 1][j] %= MOD
                if j + A[i] <= M:
                    dp[i + 1][j + A[i]] = dp[i + 1][j + A[i]] + dp[i][j]
                    if MOD:
                        dp[i + 1][j + A[i]] %= MOD

    return dp


def limited_subset_sum(N, M, A, B):
    """
    N個の整数からいくつか選び、総和をMにできるかを求める
    ただし、整数AiはBi個まで選ぶことができる

    引数:
    N (int): 要素数
    M (int): 列の長さ
    A (list): 数列
    B (list): 個数制限

    戻り値:
    dp (list): DPテーブル dp[-1][-1]がINFなら不可
    """
    dp = [[INF] * (M + 1) for _ in range(N + 1)]
    dp[0][0] = 0
    for i in range(N):
        for j in range(M + 1):
            if dp[i][j] != INF:
                dp[i + 1][j] = 0
            if j >= A[i]:
                if dp[i][j - A[i]] != INF:
                    dp[i + 1][j] = min(dp[i + 1][j], 1)
                if dp[i + 1][j - A[i]] < B[i]:
                    dp[i + 1][j] = min(dp[i + 1][j], dp[i + 1][j - A[i]] + 1)

    return dp


def max_sum_dp(A):
    """
    整数列の要素をいくつか選んだ時の、総和の最大値を
    求めるDP

    引数:
    A: 整数列

    戻り値:
    dp: DPテーブル 最後の要素が答え
    """
    N = len(A)
    dp = [0] * (N + 1)
    for i in range(N):
        dp[i + 1] = max(dp[i], dp[i] + A[i])

    return dp
