INF = 1 << 61


def knapsack(N, W, weight, value):
    """
    重さと価値が設定されたN個の品物から、重さの総和がWを超えないように
    選んだ時の、価値の総和の最大値を求めるDP
    -1なら不可

    引数:
    N (int): 品物の数
    W (int): 重さの制限
    weight (list): 重さのリスト
    value (list): 価値のリスト

    戻り値:
    dp (list): DPテーブル 最後の行の最大値が答え
    """
    dp = [[-1] * (W + 1) for _ in range(N + 1)]
    dp[0][0] = 0
    for i in range(N):
        for j in range(W + 1):
            if dp[i][j] != -1:
                dp[i + 1][j] = max(dp[i + 1][j], dp[i][j])
                if j + weight[i] <= W:
                    dp[i + 1][j + weight[i]] = max(
                        dp[i + 1][j + weight[i]], dp[i][j] + value[i]
                    )

    return dp


def knapsack_min(N, W, weight, value):
    """
    重さと価値が設定されたN個の品物から、重さの総和がWを超えないように
    選んだ時の、価値の総和の最小値を求めるDP
    INFなら不可

    引数:
    N (int): 品物の数
    W (int): 重さの制限
    weight (list): 重さのリスト
    value (list): 価値のリスト

    戻り値:
    dp (list): DPテーブル 最後の行の最大値が答え
    """
    dp = [[INF] * (W + 1) for _ in range(N + 1)]
    dp[0][0] = 0
    for i in range(N):
        for j in range(W + 1):
            if dp[i][j] != INF:
                dp[i + 1][j] = min(dp[i + 1][j], dp[i][j])
                if j + weight[i] <= W:
                    dp[i + 1][j + weight[i]] = min(
                        dp[i + 1][j + weight[i]], dp[i][j] + value[i]
                    )

    return dp
