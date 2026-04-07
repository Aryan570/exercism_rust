from math import inf
PRICES = [0, 800, 1520, 2160, 2560, 3000]
def total(books):
    v = [(0, i) for i in range(1, 6)]
    for b in books:
        v[b - 1] = (v[b - 1][0] + 1, v[b - 1][1])
    custom_sort(v)
    return f(v)

def f(v):
    cost = sum(c for c, _ in v) * 800
    distinct = sum(1 for c, _ in v if c > 0)
    for i in range(2, distinct + 1):
        v_c = v.copy()
        for j in range(i):
            c, book_id = v_c[j]
            v_c[j] = (c - 1, book_id)
        custom_sort(v_c)
        cost = min(cost, PRICES[i] + f(v_c))

    return cost

def custom_sort(arr):
    arr.sort(key=lambda x: -x[0])
