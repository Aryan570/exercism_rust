def get_day(day):
    return {
        1 : "first",
        2 : "second",
        3 : "third",
        4 : "fourth",
        5 : "fifth",
        6 : "sixth",
        7 : "seventh",
        8 : "eighth",
        9 : "ninth",
        10 : "tenth",
        11 : "eleventh",
        12 : "twelfth",
    }[day]

def get_lines():
    return [
        "a Partridge in a Pear Tree.",
        "two Turtle Doves, ",
        "three French Hens, ",
        "four Calling Birds, ",
        "five Gold Rings, ",
        "six Geese-a-Laying, ",
        "seven Swans-a-Swimming, ",
        "eight Maids-a-Milking, ",
        "nine Ladies Dancing, ",
        "ten Lords-a-Leaping, ",
        "eleven Pipers Piping, ",
        "twelve Drummers Drumming, ",
    ]
def recite(start_verse, end_verse):
    if start_verse != end_verse:
        return [recite(n, n)[0] for n in range(start_verse, end_verse + 1)]
    lines = get_lines()
    answer = [f"On the {get_day(start_verse)} day of Christmas my true love gave to me: "]

    for i in range(start_verse, 1, -1):
        answer.append(lines[i - 1])
    if start_verse == 1:
        answer.append(lines[0])
    else:
        answer.append("and " + lines[0])
    return ["".join(answer)]
