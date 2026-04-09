class TreeNode:
    def __init__(self, data, left=None, right=None):
        self.data = int(data)
        self.left = left
        self.right = right

    def __str__(self):
        return f'TreeNode(data={self.data}, left={self.left}, right={self.right})'


class BinarySearchTree:
    def __init__(self, tree_data):
        self.root = None
        for value in tree_data:
            self.root = self._insert(self.root, value)

    def _insert(self, node, value):
        value = int(value)
        if node is None:
            return TreeNode(value)
        if value <= node.data:
            node.left = self._insert(node.left, value)
        else:
            node.right = self._insert(node.right, value)
        return node

    def data(self):
        return self.root

    def sorted_data(self):
        result = []
        self._inorder(self.root, result)
        return result

    def _inorder(self, node, result):
        if not node:
            return
        self._inorder(node.left, result)
        result.append(str(node.data))
        self._inorder(node.right, result)