class Garden:
    DEFAULT_STUDENTS = [
        "Alice", "Bob", "Charlie", "David",
        "Eve", "Fred", "Ginny", "Harriet",
        "Ileana", "Joseph", "Kincaid", "Larry"
    ]
    def __init__(self, diagram, students=None):
        self.rows = diagram.split("\n")
        self.students = sorted(students if students else self.DEFAULT_STUDENTS)
        self.mpp = {
            'R': "Radishes",
            'C': "Clover",
            'G': "Grass",
            'V': "Violets"
        }

    def plants(self, student):
        idx = self.students.index(student) * 2
        result = []
        result.append(self.mpp[self.rows[0][idx]])
        result.append(self.mpp[self.rows[0][idx + 1]])
        result.append(self.mpp[self.rows[1][idx]])
        result.append(self.mpp[self.rows[1][idx + 1]])
        return result