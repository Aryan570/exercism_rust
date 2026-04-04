from collections import defaultdict

def count_words(words):
    mpp = defaultdict(int)
    ap_cnt = 0
    tmp_str = []
    for ch in words.lower():
        if ch.isalnum():
            if ap_cnt == 1:
                tmp_str.append("'")
                ap_cnt += 1
            tmp_str.append(ch)
        elif ch == ' ':
            if tmp_str:
                word = ''.join(tmp_str)
                mpp[word] += 1
                tmp_str = []
            ap_cnt = 0
        elif ch == "'":
            if tmp_str:
                ap_cnt += 1
        else:
            if tmp_str:
                word = ''.join(tmp_str)
                mpp[word] += 1
                tmp_str = []
            ap_cnt = 0
    if tmp_str:
        word = ''.join(tmp_str)
        mpp[word] += 1
    return dict(mpp)