def commands(binary_str):
    answer = []
    if binary_str[4] == '1':
        answer.append("wink")
    if binary_str[3] == '1':
        answer.append("double blink")
    if binary_str[2] == '1':
        answer.append("close your eyes")
    if binary_str[1] == '1':
        answer.append("jump")
    if binary_str[0] == '1':
        answer.reverse()
    return answer
        
    
