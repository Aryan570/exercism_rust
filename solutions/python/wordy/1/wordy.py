def answer(question):
    question = question.removeprefix("What is")
    question = question.removesuffix("?")
    question = question.replace("by", "")
    equation = question.split()
    while len(equation) > 1:
        try: 
            if equation[1] not in ["plus", "minus", "divided", "multiplied"]:
                if len(equation) >= 2 and equation[1].lstrip('-').isdigit(): 
                    raise ValueError("syntax error") 
                else: 
                    raise ValueError("unknown operation")     
            num = 0
            left = int(equation[0])
            if len(equation) < 3:
                raise ValueError("syntax error")
            right = int(equation[2])
            if equation[1] == "plus":
                num = left + right
            elif equation[1] == "minus":
                num = left - right
            elif equation[1] == "multiplied":
                num = left * right
            else:
                num = int(left / right)
            if len(equation) > 3:
                equation = [str(num)] + equation[3:]
            else:
                return int(num)
            
            
        except ValueError as e:
            if str(e) in ["unknown operation", "syntax error"]:
                raise e
            raise ValueError("syntax error")
    if len(equation) == 0:
        raise ValueError("syntax error")
    return int(equation[0])