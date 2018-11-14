from typing import List
import re


def _parse_number_list(numbers: str) -> List[int]:
    return [int(d) for d in numbers.split(',') if d.strip()]


class Instruction:
    instruction_re = re.compile(
        '^.*(?P<action>on|off|toggle)\s+(?P<start>[0-9][, 0-9]*)\s+through\s+(?P<end>[0-9][, 0-9]*)',
        re.IGNORECASE)

    def __init__(self, start: List[int], end: List[int], action: str):
        self.start = start
        self.end = end
        self.action = action

    def __eq__(self, other):
        start_eq = self.start == other.start
        end_eq = self.end == other.end
        action_eq = self.action == other.action
        return start_eq and end_eq and action_eq

    def __str__(self):
        return f"<Instruction: {self.start} through {self.end} - {self.action}>"

    def __repr__(self):
        return str(self)

    @classmethod
    def parse(cls, line: str):
        match = cls.instruction_re.match(line)
        if not match:
            raise ValueError(f"Instruction \"{line}\" could not be parsed!")
        action = match.group('action').strip().lower()
        start = _parse_number_list(match.group('start'))
        end = _parse_number_list(match.group('end'))

        if cls._dimensions_invalid(start, end):
            raise ValueError(f"Instruction \"{line}\" must have valid coordinate pair!")

        return Instruction(start, end, action)

    @classmethod
    def _dimensions_invalid(cls, start: List[int], end: List[int]):
        return len(start) != len(end) or any([d for d in start + end if d < 0])
