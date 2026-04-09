def largest_product(digits: str, span: int) -> int:
    if span < 0:
        raise ValueError("span must not be negative")
    if span > len(digits):
        raise ValueError("span must not exceed string length")
    if any(not ch.isdigit() for ch in digits):
        raise ValueError("digits input must only contain digits")
    if span == 0:
        return 1
    nums = [int(ch) for ch in digits]
    result = 1
    ans = 0
    left = 0
    right = 0
    while right < span:
        result *= nums[right]
        right += 1
    ans = max(ans, result)
    while right < len(nums):
        if nums[left] == 0:
            result = 1
            for i in range(left + 1, right):
                result *= nums[i]
        else:
            result //= nums[left]
        result *= nums[right]
        ans = max(ans, result)
        left += 1
        right += 1
    return ans