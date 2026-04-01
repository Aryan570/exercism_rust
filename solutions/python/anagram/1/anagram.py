def find_anagrams(word, candidates):
    find_word = ''.join(sorted(word.lower()))
    ans = []
    for c in candidates:
        if find_word == ''.join(sorted(c.lower())) and (c.lower() != word.lower()):
            ans.append(c)
    return ans
            
        
