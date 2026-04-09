class Luhn:
    def __init__(self, card_num):
        self.code = card_num

    def valid(self):
        flag = False
        cnt = 0
        total = 0
        for ch in reversed(self.code):
            if ch != ' ' and not ch.isdigit():
                return False
            if ch == ' ':
                continue
            tmp = int(ch)
            total += tmp
            if flag:
                total -= tmp
                doubled = 2 * tmp
                if doubled > 9:
                    total += doubled - 9
                else:
                    total += doubled
            flag = not flag
            cnt += 1
        return total % 10 == 0 and cnt > 1