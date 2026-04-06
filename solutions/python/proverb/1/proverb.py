def proverb(*input_data, qualifier=None):
    if not input_data:
        return []
    lines = []
    for a, b in zip(input_data, input_data[1:]):
        lines.append(f"For want of a {a} the {b} was lost.")
    first = input_data[0]
    if qualifier:
        lines.append(f"And all for the want of a {qualifier} {first}.")
    else:
        lines.append(f"And all for the want of a {first}.")
    return lines