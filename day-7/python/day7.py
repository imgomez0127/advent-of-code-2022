from typing import Any
import dataclasses


@dataclasses.dataclass
class Node:
    name: str = ''
    is_dir: bool = False
    size: int = 0
    children: list[Any] = dataclasses.field(default_factory=list)
    parent: Any = None

    def find_child(self, name: str):
        for child in self.children:
            if child.name == name:
                return child
        return None

    def str_helper(self, node, depth):
        filetype = '(dir)' if node.is_dir else '(file)'
        cur_str = ' ' * depth * 2 + f'- {node.name} {filetype} (size {node.size})\n'
        for child in node.children:
            cur_str += self.str_helper(child, depth+1)
        return cur_str

    def __str__(self):
        return self.str_helper(self, 0)


def parse():
    root = Node(name='/', is_dir=True)
    cur = root
    with open('input.txt') as f:
        for line in f:
            match line.strip().split():
                case ['$', 'cd', '/']:
                    cur = root
                case ['$', 'cd', '..']:
                    cur = cur.parent
                case ['$', 'cd', directory]:
                    cur = cur.find_child(directory)
                    if not cur or not cur.is_dir:
                        raise ValueError('Found node either does not '
                                         'exist or is not a directory.')
                case ['$', 'ls']:
                    pass
                case ['dir', directory]:
                    cur.children.append(
                        Node(name=directory, is_dir=True, parent=cur))
                case [size, file_name]:
                    cur.children.append(
                        Node(name=file_name, size=int(size), parent=cur))
                case _:
                    raise ValueError('Syntax Error could not recognize value')

    return root


class Solution:

    def __init__(self):
        self.answer = 0

    def task1(self, directory: Node) -> int:
        dir_total = 0
        for node in directory.children:
            if not node.is_dir:
                dir_total += node.size
            else:
                dir_total += self.task1(node)
        directory.size = dir_total
        if dir_total < 100000 and directory.name != '/':
            self.answer += dir_total
        return dir_total

    def task2(self, directory: Node, update_size: int) -> int:
        delete_size = directory.size
        for child in directory.children:
            if child.is_dir and child.size >= update_size:
                delete_size = min(delete_size, child.size, self.task2(child, update_size))
        return delete_size


fs = parse()
sol = Solution()
sol.task1(fs)
print(sol.answer)
update_size = 30000000
available_space = 70000000 - fs.size
print(f'needed_space: {update_size - available_space}')
print(f'delete_size: {sol.task2(fs, update_size - available_space)}')
