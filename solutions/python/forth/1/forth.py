class StackUnderflowError(Exception):
    def __init__(self, message="Insufficient number of items in stack"):
        super().__init__(message)

class Forth:
    ADD = "+"
    SUBTRACT = "-"
    MULTIPLY = "*"
    DIVIDE = "/"
    DROP = "drop"
    DUP = "dup"
    SWAP = "swap"
    OVER = "over"
    START = ":"
    END = ";"
    def __init__(self):
        self.stack = []
        self.definitions = {}

    def _replace_key_word(self, word, old_val):
        for key, val in list(self.definitions.items()):
            tokens = val.split()
            if word in tokens:
                self.definitions[key] = " ".join(
                    old_val if tok == word else tok for tok in tokens
                )

    def _arithmatic(self, operation):
        if len(self.stack) < 2:
            raise StackUnderflowError("Insufficient number of items in stack")
        right = self.stack.pop()
        left = self.stack.pop()
        if operation == self.DIVIDE and right == 0:
            raise ZeroDivisionError("divide by zero")
        if operation == self.ADD:
            self.stack.append(left + right)
        elif operation == self.SUBTRACT:
            self.stack.append(left - right)
        elif operation == self.MULTIPLY:
            self.stack.append(left * right)
        elif operation == self.DIVIDE:
            self.stack.append(left // right)
        else:
            raise ValueError("undefined operation")

    def _stack_operations(self, operation):
        if operation == self.DROP:
            if not self.stack:
                raise StackUnderflowError("Insufficient number of items in stack")
            self.stack.pop()
        elif operation == self.DUP:
            if not self.stack:
                raise StackUnderflowError("Insufficient number of items in stack")
            self.stack.append(self.stack[-1])
        elif operation == self.SWAP:
            if len(self.stack) < 2:
                raise StackUnderflowError("Insufficient number of items in stack")
            self.stack[-1], self.stack[-2] = self.stack[-2], self.stack[-1]
        elif operation == self.OVER:
            if len(self.stack) < 2:
                raise StackUnderflowError("Insufficient number of items in stack")
            self.stack.append(self.stack[-2])
        else:
            raise ValueError("undefined operation")

    def eval(self, input_data):
        if isinstance(input_data, (list, tuple)):
            input_data = " ".join(input_data)
        running_add_word = False
        key_word = ""
        value_of_word = []
        for token in input_data.split():
            token = token.lower()
            if token == self.START and not running_add_word:
                running_add_word = True
                key_word = ""
                value_of_word = []
                continue
            if token == self.END and running_add_word:
                running_add_word = False
                if key_word.lstrip("-").isdigit():
                    raise ValueError("illegal operation")
                definition = " ".join(value_of_word)
                old_value = self.definitions.get(key_word)
                self.definitions[key_word] = definition
                if old_value is not None:
                    self._replace_key_word(key_word, old_value)
                continue
            if running_add_word:
                if not key_word:
                    if token.lstrip("-").isdigit():
                        raise ValueError("illegal operation")
                    key_word = token
                else:
                    value_of_word.append(token)
                continue
            if token in self.definitions:
                self.eval(self.definitions[token])
            elif token.lstrip("-").isdigit():
                self.stack.append(int(token))
            elif token in {self.ADD, self.SUBTRACT, self.MULTIPLY, self.DIVIDE}:
                self._arithmatic(token)
            elif token in {self.DROP, self.DUP, self.SWAP, self.OVER}:
                self._stack_operations(token)
            else:
                raise ValueError("undefined operation")
        if running_add_word:
            raise ValueError("illegal operation")


def evaluate(input_data):
    forth = Forth()
    forth.eval(input_data)
    return forth.stack