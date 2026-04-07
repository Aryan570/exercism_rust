def saddle_points(matrix):
    if not matrix:
        return []
    
    row_len = len(matrix)
    col_len = len(matrix[0])
    for row in matrix:
        if len(row) != col_len:
            raise ValueError("irregular matrix")
    result = []
    for i in range(row_len):
        for j in range(col_len):
            curr = matrix[i][j]
            if any(curr < matrix[i][k] for k in range(col_len)):
                continue
            if any(curr > matrix[k][j] for k in range(row_len)):
                continue
            result.append({"row": i + 1, "column": j + 1})
    return result
