class School:
    def __init__(self):
        self.mpp = {}
        self._added_log = []

    def add_student(self, name, grade):
        if grade not in self.mpp:
            self.mpp[grade] = set()
        for students in self.mpp.values():
            if name in students:
                self._added_log.append(False)
                return
        self.mpp[grade].add(name)
        self._added_log.append(True)

    def roster(self):
        result = []
        for grade in sorted(self.mpp.keys()):
            result.extend(sorted(self.mpp[grade]))
        return result

    def grade(self, grade_number):
        if grade_number in self.mpp:
            return sorted(self.mpp[grade_number])
        return []

    def added(self):
        return self._added_log