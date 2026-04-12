def line_up(name, number):
    tag = "th"
    last_digit = number % 10
    if last_digit == 1 and number % 100 != 11:
        tag = "st"
    if last_digit == 2 and number % 100 != 12:
        tag = "nd"
    if last_digit == 3 and number % 100 != 13:
        tag = "rd"
    return f"{name}, you are the {number}{tag} customer we serve today. Thank you!"