def sum_of_multiples(limit, factors):
    multiples = set()
    for factor in factors:
        if factor == 0:
            continue
        for num in range(factor, limit, factor):
            multiples.add(num)

    return sum(multiples)
