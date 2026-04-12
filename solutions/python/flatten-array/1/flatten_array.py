def flatten(iterable):
    answer = []
    for curr in iterable:
        if type(curr) is list:
            answer.extend(flatten(curr))
        elif type(curr) is int:
            answer.append(curr)
    return answer
