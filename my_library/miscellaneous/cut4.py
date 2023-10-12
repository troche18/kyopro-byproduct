def rotate90(matrix):
    return list(zip(*matrix[::-1]))


def cut(matrix):
    cnt = 0
    for i in range(len(matrix)):
        if "#" in matrix[i]:
            break
        else:
            cnt += 1

    return matrix[cnt:]


def cut4(matrix):
    for _ in range(4):
        matrix = cut(matrix)
        matrix = rotate90(matrix)
    return matrix
