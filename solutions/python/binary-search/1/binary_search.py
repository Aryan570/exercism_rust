def find(search_list, value):
    left = 0
    right = len(search_list) - 1
    while right >= left:
        mid = (left + right) // 2
        if value <= search_list[mid]:
            right = mid - 1
        else:
            left = mid + 1
    if left < len(search_list) and search_list[left] == value:
        return left
    raise ValueError("value not in array")
        
        
