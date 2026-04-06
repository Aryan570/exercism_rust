def transpose(text):
    rows = text.splitlines()
    if not rows:
        return ""
    row_cnt = len(rows)
    col_cnt = max(len(row) for row in rows)
    matrix = [[' ' for _ in range(row_cnt)] for _ in range(col_cnt)]
    for i in range(row_cnt):
        for j in range(len(rows[i])):
            matrix[j][i] = rows[i][j]
    result = []
    for i in range(col_cnt):
        row = matrix[i]
        end = row_cnt
        while end > 0 and row[end - 1] == ' ' and len(rows[end - 1]) <= i:
            end -= 1
        result.append("".join(row[:end]))
    return "\n".join(result)