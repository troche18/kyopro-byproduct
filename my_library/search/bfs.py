from collections import *

INF = 999
N = 100

edges = [[1, 2], [1, 3], [2, 4]]
G = defaultdict(set)

for i, j in edges:
    G[i].add(j)
    G[j].add(i)


# グラフ//////////////////////////////////////////////////////////////////////
def bfs(G, start):
    """
    グラフで幅優先探索をする関数
    seenとdistを配列で管理

    引数:
    G (defaultdict): グラフ
    start (int): 始点

    戻り値:
    dist (list): 始点からの距離
    """
    que = deque()
    seen = [0] * (N + 1)
    dist = [INF] * (N + 1)

    que.append(start)
    seen[start] = True
    dist[start] = 0
    while que:
        for _ in range(len(que)):
            pop = que.popleft()
            for i in G[pop]:
                if seen[i] == False:
                    seen[i] = True
                    dist[i] = dist[pop] + 1
                    que.append(i)

    return dist


def bfs(G, start):
    """
    グラフで幅優先探索をする関数
    seenを集合、distを辞書で管理
    配列で管理するより遅め

    引数:
    G (defaultdict): グラフ
    start (int): 始点

    戻り値:
    dist (dict): 始点からの距離
    """
    que = deque()
    seen = set()
    dist = {}

    que.append(start)
    seen.add(start)
    dist[start] = 0
    while que:
        for _ in range(len(que)):
            pop = que.popleft()
            for i in G[pop]:
                if i not in seen:
                    seen.add(i)
                    dist[i] = dist.get(pop, 0) + 1
                    que.append(i)

    return dist


# グリッド/////////////////////////////////////////////////////////////////////
H = 3
W = 3
grid = [[".", ".", "#", "#"], ["#", ".", "#", "#"], ["#", ".", ".", "#"]] + [
    ["#"] * (W + 1)
]


def bfs(grid, start):
    """
    グリッドで幅優先探索をする
    seenとdistを二次元配列で管理

    引数:
    grid (list): グリッド
    start (tuple): 始点の座標

    戻り値:
    dist (list): 始点からの距離
    """
    dir4 = [[1, 0], [0, 1], [-1, 0], [0, -1]]
    que = deque()
    seen = [[0] * (W + 1) for _ in range(H + 1)]
    dist = [[INF] * (W + 1) for _ in range(H + 1)]

    que.append(start)
    seen[start[0]][start[1]] = 1
    dist[start[0]][start[1]] = 0
    while que:
        for _ in range(len(que)):
            pi, pj = que.popleft()
            for di, dj in dir4:
                if seen[pi + di][pj + dj] == 0 and grid[pi + di][pj + dj] == ".":
                    seen[pi + di][pj + dj] = 1
                    dist[pi + di][pj + dj] = dist[pi][pj] + 1
                    que.append((pi + di, pj + dj))

    return dist


def bfs(grid, start):
    """
    グリッドで幅優先探索をする
    seenを集合、distを辞書で管理
    配列で管理するより遅め

    引数:
    grid (list): グリッド
    start (tuple): 始点の座標

    戻り値:
    dist (dict): 始点からの距離
    """
    dir4 = [[1, 0], [0, 1], [-1, 0], [0, -1]]
    que = deque()
    seen = set()
    dist = {}

    que.append(start)
    seen.add(start)
    dist[start] = 0
    while que:
        for _ in range(len(que)):
            pi, pj = que.popleft()
            for di, dj in dir4:
                if (pi + di, pj + dj) not in seen and grid[pi + di][pj + dj] == ".":
                    seen.add((pi + di, pj + dj))
                    dist[(pi + di, pj + dj)] = dist.get((pi, pj), 0) + 1
                    que.append((pi + di, pj + dj))

    return dist
