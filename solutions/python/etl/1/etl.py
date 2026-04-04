def transform(legacy_data):
    ans = {}
    for key, value in legacy_data.items():
        for ele in value:
            ans[chr(ord(ele) + 32)] = key
    return ans
