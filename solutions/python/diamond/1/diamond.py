def rows(letter):
    spaces = (ord(letter) - ord('A')) * 2 + 1
    left = 0
    right = spaces - 1
    answer = []
    while right >= left:
        tmp_list = [' '] * (spaces)
        tmp_list[left] = letter
        tmp_list[right] = letter
        letter = chr(ord(letter) - 1)
        left += 1
        right -= 1
        answer.append("".join(tmp_list))

    copy_answer = answer.copy()
    rem = answer[1:]
    rem.reverse()
    return rem + copy_answer 
