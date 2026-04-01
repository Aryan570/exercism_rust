def encode(numbers):
    answer = []
    for number in reversed(numbers):
        answer.append((number & 0x7F))
        number = number >> 7
        while number > 0:
            answer.append((number & 0x7F) | 0x80)
            number = number >> 7
    answer.reverse()
    return answer


def decode(bytes):
    answer = []
    has_next = False
    number = 0
    for byte in bytes:
        number = (number << 7) | (byte & 0x7F)
        if byte & 0x80:
            has_next = True
        else:
            answer.append(number)
            number = 0
            has_next = False

    if has_next:
        raise ValueError("incomplete sequence")
    return answer
        
    
        
        
