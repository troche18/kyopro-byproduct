def topological_sort(G, node_list, out_cnt):
    """
    トポロジカルソート
    戻り値の要素数が頂点数より少なければサイクルがある

    引数:
    G: 有向辺を逆向きに張ったグラフ
    node_list: グラフを構成する頂点
    out_cnt: 頂点の出次数

    戻り値:
    return: 出次数が0になった順に頂点を格納したリスト
    """
    que = deque([i for i in node_list if not out_cnt[i]])
    ret = []
    while que:
        pop = que.popleft()
        ret.append(pop)
        for v in G[pop]:
            out_cnt[v] -= 1
            if out_cnt[v] == 0:
                que.append(v)

    return ret
