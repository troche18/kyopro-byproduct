# 手の定義
# R, S, P = list("RSP")
R, S, P = list("GCP")
# R, S, P = list("rsp")
# R, S, P = list("gcp")


def judge(a, b):
    """
    aが勝ち -> 1
    aが負け -> 0
    あいこ  -> -1
    """
    if a == b:
        return -1
    if a == R and b == P:
        return 0
    if a == S and b == R:
        return 0
    if a == P and b == S:
        return 0
    return 1
