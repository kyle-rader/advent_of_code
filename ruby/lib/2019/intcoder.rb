class Intcoder
    def initialize(program)
        @program = program
            .split(",")
            .map {|c| c.strip.to_i }
        @opcode_ptr = 0
    end

    def run(noun = nil, verb = nil)
        set_inputs(noun, verb) if noun && verb
        while opcode != 99 do
            process
            step
        end
        @program[0]
    end

    def set_inputs(noun, verb)
        @program[1] = noun
        @program[2] = verb
    end

    def opcode
        @program[@opcode_ptr]
    end

    def step
        @opcode_ptr += 4
    end

    def process
        case opcode
        when 1
            add
        when 2
            mult
        else
            raise "Oops, we don't know how to handle opcode '#{opcode}'"
        end
    end

    def add
        set_val(arg1 + arg2)
    end

    def mult
        set_val(arg1 * arg2)
    end

    def set_val(val)
        @program[target] = val
    end

    def target
        @program[@opcode_ptr + 3]
    end

    def arg1
        @program[@program[@opcode_ptr + 1]]
    end

    def arg2
        @program[@program[@opcode_ptr + 2]]
    end
end
