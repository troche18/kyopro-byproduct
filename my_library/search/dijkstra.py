def Dijkstra(G, num_nodes, start_node):
    """
    引数:
    G (defaultdict): 値が(頂点番号,コスト)のdefaultdict
    num_nodes (int): 頂点の数
    start_node (int) :始点の番号

    戻り値:
    dist (list): 始点からの距離
    """
    heap = [(0, start_node)]
    dist = [INF] * num_nodes
    dist[start_node] = 0
    flag = [False] * num_nodes
    cnt = 0

    while heap and cnt < num_nodes:
        popC, popV = heappop(heap)
        if flag[popV]:
            continue

        for v, c in G[popV]:
            if dist[v] > popC + c:
                dist[v] = popC + c
                heappush(heap, (dist[v], v))
        flag[popV] = True
        cnt += 1

    return dist
