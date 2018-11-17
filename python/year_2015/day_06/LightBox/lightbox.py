from typing import List


def follow_instructions(instructions):
    lights_on = set()

    for inst in instructions:
        for light in inst.lights():
            is_on = light in lights_on

            if inst.action == 'on' or (inst.action == 'toggle' and not is_on):
                lights_on.add(light)

            elif (inst.action == 'off' and is_on) or (inst.action == 'toggle' and is_on):
                lights_on.remove(light)

    return len(lights_on)
