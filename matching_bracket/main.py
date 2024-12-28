def is_valid_string(input: str) -> bool:
    if len(input) % 2 != 0:
        return False
    valid = True
    for i in range(len(input)):
        match input[i]:
            case '(':
                valid = validate(')', input, i)
            case '{':
                valid = validate('}', input, i)
            case '[':
                valid = validate(']', input, i)
    return valid

def validate(char: str, input: str, i: int) -> bool:
    # get the index of char
    close_index = input.find(char)
    if close_index == -1:
        return False
    # get substring between open_bracket and close_bracket
    sub_string = input[i+1:close_index]
    if len(sub_string) % 2 != 0:
        return False
    return True

print(is_valid_string("(]")) # False
print(is_valid_string("{}")) # True
print(is_valid_string("[]")) # True
print(is_valid_string("([)]")) # False
print(is_valid_string("{[]}")) # True
print(is_valid_string("()[]{}")) # True
print(is_valid_string("([{}])")) # True
print(is_valid_string("(){}]")) # False