ans = []
def backtrack(idx, coins, target, tmp):
    global ans
    if len(ans) != 0 and len(tmp) > len(ans):
        return
    if idx >= len(coins):
        if target == 0:
            if len(ans) == 0 or len(tmp) < len(ans):
                ans = tmp[:]
        return

    if target >= coins[idx]:
        tmp.append(coins[idx])
        backtrack(idx, coins, target - coins[idx], tmp)
        tmp.pop()
    backtrack(idx + 1, coins, target, tmp) 
    
def find_fewest_coins(coins, target):
    global ans
    if target < 0:
        raise ValueError("target can't be negative")
    if target == 0:
        return []
    coins.sort(reverse=True)
    ans = []
    backtrack(0, coins, target, [])
    if len(ans) == 0:
        raise ValueError("can't make target with given coins")
    ans.reverse()
    return ans
