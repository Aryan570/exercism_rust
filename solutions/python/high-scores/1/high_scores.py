class HighScores:
    def __init__(self, scores):
        self.scores = scores
        self.latest_score = scores[-1] if scores else None

    def latest(self):
        return self.latest_score

    def personal_best(self):
        return max(self.scores) if self.scores else None

    def personal_top_three(self):
        return sorted(self.scores, reverse=True)[:3]