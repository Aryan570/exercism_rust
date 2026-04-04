def tally(rows):
    win = {}
    loss = {}
    draw = {}
    teams = {}
    answer = ["Team                           | MP |  W |  D |  L |  P"]
    for match in rows:
        details = match.split(';')
        if details[2] == "win":
            win[details[0]] = win.get(details[0], 0) + 1
            loss[details[1]] = loss.get(details[1], 0) + 1
        if details[2] == "loss":
            win[details[1]] = win.get(details[1], 0) + 1
            loss[details[0]] = loss.get(details[0], 0) + 1
        if details[2] == "draw":
            draw[details[0]] = draw.get(details[0], 0) + 1
            draw[details[1]] = draw.get(details[1], 0) + 1
        teams[details[0]] = 0
        teams[details[1]] = 0
    table_data = []
    for team in teams:
        total_wins = win.get(team, 0)
        total_loss = loss.get(team, 0)
        total_draw = draw.get(team, 0)
        total_match = total_wins + total_loss + total_draw
        total_pt = 3 * total_wins + total_draw
        table_data.append((team, total_match, total_wins, total_draw, total_loss, total_pt))
    table_data.sort(key=lambda x: (-x[5], x[0]))
    for team, total_match, total_wins, total_draw, total_loss, total_pt in table_data:
        answer.append(
            f"{team:<30} | {total_match:>2} | {total_wins:>2} | {total_draw:>2} | {total_loss:>2} | {total_pt:>2}"
        )
    return answer
            
        
