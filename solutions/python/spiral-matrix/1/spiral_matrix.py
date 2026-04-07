def spiral_matrix(size):
    left, right = 0, size - 1
    top, bottom = 0, size - 1
    cnt = 1
    v = [[0] * size for _ in range(size)]
    while left <= right and top <= bottom:
        # Top row
        for i in range(left, right + 1):
            v[top][i] = cnt
            cnt += 1
        top += 1
        # Right column
        for i in range(top, bottom + 1):
            v[i][right] = cnt
            cnt += 1
        right -= 1
        # Bottom row
        if top <= bottom:
            for i in range(right, left - 1, -1):
                v[bottom][i] = cnt
                cnt += 1
            bottom -= 1
        # Left column
        if left <= right:
            for i in range(bottom, top - 1, -1):
                v[i][left] = cnt
                cnt += 1
            left += 1
    return v