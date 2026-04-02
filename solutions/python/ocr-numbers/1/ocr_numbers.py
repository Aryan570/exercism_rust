def convert(input):
    patterns = {
        " _ | ||_|   ": "0",
        "     |  |   ": "1",
        " _  _||_    ": "2",
        " _  _| _|   ": "3",
        "   |_|  |   ": "4",
        " _ |_  _|   ": "5",
        " _ |_ |_|   ": "6",
        " _   |  |   ": "7",
        " _ |_||_|   ": "8",
        " _ |_| _|   ": "9",
    }
    lines = input
    if len(lines) % 4 != 0:
        raise ValueError("Number of input lines is not a multiple of four")
    for line in lines:
        if len(line) % 3 != 0:
            raise ValueError("Number of input columns is not a multiple of three")
    result = []
    for block_start in range(0, len(lines), 4):
        block = lines[block_start:block_start + 4]
        digit_count = len(block[0]) // 3
        current = []
        for d in range(digit_count):
            key = "".join(row[d * 3:(d + 1) * 3] for row in block)
            current.append(patterns.get(key, "?"))

        result.append("".join(current))
    return ",".join(result)