NUMBERS = [
    "no", "one", "two", "three", "four",
    "five", "six", "seven", "eight", "nine", "ten"
]

def verse(n: int):
    current = NUMBERS[n].capitalize()
    next_num = NUMBERS[n - 1] if n > 0 else "no"
    bottle = "bottle" if n == 1 else "bottles"
    next_bottle = "bottle" if n - 1 == 1 else "bottles"
    return [
        f"{current} green {bottle} hanging on the wall,",
        f"{current} green {bottle} hanging on the wall,",
        f"And if one green bottle should accidentally fall,",
        f"There'll be {next_num} green {next_bottle} hanging on the wall.",
    ]
def recite(start, take=1):
    end = start - take + 1
    verses = []
    for n in range(start, end - 1, -1):
        for v in verse(n):
            verses.append(v)
        verses.append("")
    return verses[:-1]