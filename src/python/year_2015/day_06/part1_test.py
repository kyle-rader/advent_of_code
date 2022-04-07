from year_2015.day_06.LightBox import Instruction, follow_instructions


inst1 = """
turn on 489,959 through 759,964
turn off 820,516 through 871,914
turn off 427,423 through 929,502
turn on 774,14 through 977,877
"""

def test_part1():
    inst = [Instruction.parse(l.strip()) for l in inst1.splitlines(keepends=False) if l.strip()]

    (lights_on, brightness) = follow_instructions(inst)

    assert 177882 == lights_on

