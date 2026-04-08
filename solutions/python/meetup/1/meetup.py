from datetime import date, timedelta


# subclassing the built-in ValueError to create MeetupDayException
class MeetupDayException(ValueError):
    def __init__(self):
        super().__init__("That day does not exist.")


def meetup(year, month, week, day_of_week):
    weekdays = {
        "Monday": 0,
        "Tuesday": 1,
        "Wednesday": 2,
        "Thursday": 3,
        "Friday": 4,
        "Saturday": 5,
        "Sunday": 6,
    }
    week_offsets = {
        "first": 0,
        "second": 7,
        "third": 14,
        "fourth": 21,
        "fifth": 28,
    }
    target_weekday = weekdays[day_of_week]
    if week == "teenth":
        start_day, end_day = 13, 19
        for day in range(start_day, end_day + 1):
            d = date(year, month, day)
            if d.weekday() == target_weekday:
                return d
        raise MeetupDayException()
    if week == "last":
        first_day_next_month = date(year + (month == 12), 1 if month == 12 else month + 1, 1)
        last_day = first_day_next_month - timedelta(days=1)
        for day in range(last_day.day, 0, -1):
            d = date(year, month, day)
            if d.weekday() == target_weekday:
                return d
        raise MeetupDayException()
    start_day = 1 + week_offsets[week]
    for day in range(start_day, start_day + 7):
        try:
            d = date(year, month, day)
        except ValueError:
            break
        if d.weekday() == target_weekday:
            return d

    raise MeetupDayException()