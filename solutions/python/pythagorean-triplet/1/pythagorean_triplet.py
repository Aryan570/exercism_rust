def triplets_with_sum(sum):
    result = set()
    for a in range(3, sum // 3):
        numerator = sum * (sum - 2 * a)
        denominator = 2 * (sum - a)
        if denominator == 0:
            continue
        if numerator % denominator != 0:
            continue
        b = numerator // denominator
        c = sum - a - b
        if a < b < c:
            result.add((a, b, c))
    return [list(t) for t in result]