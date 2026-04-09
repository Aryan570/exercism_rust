class Matrix:
    def __init__(self, matrix_string):
        self.v = matrix_string.strip().split("\n")

    def row(self, index):
        if index > len(self.v) or index <= 0:
            return None
        return [int(x) for x in self.v[index - 1].split()]

    def column(self, index):
        ans = []
        for row in self.v:
            nums = row.split()
            if index <= len(nums):
                ans.append(int(nums[index - 1]))
        return ans if ans else None