from typing import List

from numpy import uint16

AND = 'AND'
OR = 'OR'
RSHIFT = 'RSHIFT'
LSHIFT = 'LSHIFT'
NOT = 'NOT'


class Circuit(object):
    def __init__(self, steps:str=None):
        self.state = {}

        if steps is not None:
            self.execute_steps(steps.splitlines(keepends=False))

    def __getitem__(self, item):
        return self.state[item]

    def execute_steps(self, steps: List[str]):
        for step in filter(None, steps):
            [expression, target] = step.split('->')
            self.state[target.strip().lower()] = self.eval(expression)

    def eval(self, expression: str):
        expression = expression.strip()

        if AND in expression:
            [op1, op2] = self._get_operands(expression, AND)
            return uint16(op1) & uint16(op2)
        elif OR in expression:
            [op1, op2] = self._get_operands(expression, OR)
            return uint16(op1) | uint16(op2)
        elif RSHIFT in expression:
            [op1, op2] = self._get_operands(expression, RSHIFT)
            return uint16(op1) >> uint16(op2)
        elif LSHIFT in expression:
            [op1, op2] = self._get_operands(expression, LSHIFT)
            return uint16(op1) << uint16(op2)
        elif NOT in expression:
            [_, op2] = self._get_operands(expression, NOT)
            return ~uint16(op2)
        elif expression.isdigit():
            return uint16(expression)
        else:
            raise ValueError(f"Could not parse operation '{expression}'")

    def _get_operands(self, expression: str, op: str):
        operands = [op.strip() for op in expression.split(op)]

        for i, op in enumerate(operands):
            if op in self.state:
                operands[i] = self.state[op]
            elif op is not None and len(op) > 0:
                operands[i] = uint16(op)
            else:
                operands[i] = None

        return operands
