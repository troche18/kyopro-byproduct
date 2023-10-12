# 確立mod
def modP(p, q):
    return p * pow(q, MOD - 2, MOD)


def mod_inv(x):
    return pow(x, MOD - 2, MOD)


def mod_perm(n, k):
    ret = 1
    for i in range(k):
        ret = (ret * (n - i)) % MOD
    return ret


def mod_comb(n, r):
    a = mod_perm(n, r)
    b = mod_perm(r, r)
    return (a * mod_inv(b)) % MOD


def combr(n, r):
    return mod_comb(n + r - 1, r)
