# 三角形の面積を求める&3点が三角形を成すかどうか（返り値が0以上か判定すればいい）
def is_triangle(a, b, c):
    a = np.array(a)
    b = np.array(b)
    c = np.array(c)
    ab = b - a
    ac = c - a
    cross = np.cross(ab, ac)
    return 0.5 * np.linalg.norm(cross)
