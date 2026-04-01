def annotate(garden):
    if not garden:
        return []
    row_length = len(garden[0])
    for row in garden:
        if len(row) != row_length:
            raise ValueError("The board is invalid with current input.")
        for ch in row:
            if ch not in (' ', '*'):
                raise ValueError("The board is invalid with current input.")

    tmp = [list(row) for row in garden]
    answer = []
    for i, row in enumerate(tmp):
        tmp_str = ""
        for j, ch in enumerate(row):
            if ch == ' ':
                neighbours = 0
                for ii in range(-1, 2):
                    for jj in range(-1, 2):
                        r = i + ii
                        c = j + jj
                        if (
                            0 <= r < len(tmp)
                            and 0 <= c < len(row)
                            and tmp[r][c] == '*'
                        ):
                            neighbours += 1

                tmp_str += str(neighbours) if neighbours > 0 else ' '
            else:
                tmp_str += '*'
        answer.append(tmp_str)

    return answer
