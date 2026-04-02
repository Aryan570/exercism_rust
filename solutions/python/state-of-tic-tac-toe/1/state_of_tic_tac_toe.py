def cnt_x_o(board):
    cntX = 0
    cntO = 0
    for row in board:
        for ch in row:
            if ch == 'X':
                cntX += 1
            if ch == 'O':
                cntO += 1
    return cntX - cntO

def who_won(board, c):
    row_str = c + c + c
    if row_str == board[0] or row_str == board[1] or row_str == board[2]:
        return True
    if board[0][0] == c and board[1][0] == c and board[2][0] == c:
        return True
    if board[0][1] == c and board[1][1] == c and board[2][1] == c:
        return True
    if board[0][2] == c and board[1][2] == c and board[2][2] == c:
        return True
    if board[0][0] == c and board[1][1] == c and board[2][2] == c:
        return True
    if board[0][2] == c and board[1][1] == c and board[2][0] == c:
        return True
    return False

def spaces_cnt(board):
    cnt = 0
    for row in board:
        for ch in row:
            if ch == ' ':
                cnt += 1 
    return cnt == 0
        
def gamestate(board):
    if cnt_x_o(board) < 0:
        raise ValueError("Wrong turn order: O started")
    if cnt_x_o(board) > 1:
        raise ValueError("Wrong turn order: X went twice")
    did_x_win = who_won(board, 'X')
    did_o_win = who_won(board, 'O')
    if did_x_win and did_o_win:
        raise ValueError("Impossible board: game should have ended after the game was won")
    if (not did_x_win) and (not did_o_win) and (not spaces_cnt(board)):
        return "ongoing"
    if (not did_x_win) and (not did_o_win):
        return "draw"
    return "win"
