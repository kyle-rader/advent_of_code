class Intcoder
  attr_reader :opcode_ptr, :step_by, :program, :input, :output,
    :opcode, :arg_mode
  
  def initialize(program)
    @program = program
      .split(",")
      .map {|c| c.strip.to_i }
    @opcode_ptr = 0
    @input = []
    @output = []
  end

  def run(noun = nil, verb = nil)
      set_inputs(noun, verb) if noun && verb
      while opcode != 99 do
          process
          step
      end
      program[0]
  end

  def opcode
    program[opcode_ptr] % 100
  end

  def in(val)
    @input << val
  end

  def set_inputs(noun, verb)
    @program[1] = noun
    @program[2] = verb
  end

  def step
    @opcode_ptr += step_by
  end

  def process
    case opcode
    when 1
      do_add
      set_step 4
    when 2
      do_mult
      set_step 4
    when 3
      do_input
      set_step 2
    when 4
      do_output
      set_step 2
    else
      raise "Oops, we don't know how to handle opcode '#{opcode}'"
    end
  end

  def set_step(n)
    @step_by = n 
  end

  def do_add
    set_val(arg1 + arg2)
  end

  def do_mult
    set_val(arg1 * arg2)
  end

  def do_input
    @program[program[opcode_ptr + 1]] = input.pop
  end

  def do_output
    @output << program[program[opcode_ptr + 1]]
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
