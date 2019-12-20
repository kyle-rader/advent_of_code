class Intcoder

  def self.get_modes(m)
    modes = []
    loop do
      m, r = m.divmod(10)
      if (m == 0 && r == 0)
        break
      end
      modes << r
    end
    modes
  end

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
          clear
      end
      program[0]
  end

  def clear
    @_opcode = nil
    @_arg_modes = nil
  end

  def opcode
    @_opcode ||= program[opcode_ptr] % 100
  end

  def arg_modes
    @_arg_modes ||= Intcoder.get_modes(program[opcode_ptr] / 100)
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
    when 2
      do_mult
    when 3
      do_input
    when 4
      do_output
    when 7
      do_less_than
    when 8
      do_equals
    else
      raise "Oops, we don't know how to handle opcode '#{opcode}'"
    end
  end

  def set_step(n)
    @step_by = n
  end

  def do_add
    set_val(arg(0) + arg(1))
    set_step 4
  end

  def do_mult
    set_val(arg(0) * arg(1))
    set_step 4
  end

  def do_input
    @program[program[opcode_ptr + 1]] = input.pop
    set_step 2
  end

  def do_output
    @output << program[program[opcode_ptr + 1]]
    set_step 2
  end

  def do_less_than
    set_val(true?(arg(0) < arg(1)))
    set_step 4
  end

  def do_equals
    set_val(true?(arg(0) == arg(1)))
    set_step 4
  end

  def true?(val)
    val ? 1 : 0
  end

  def set_val(val)
    @program[program[opcode_ptr + 3]] = val
  end

  def arg(i)
    immediate_val = program[opcode_ptr + i + 1]
    case arg_modes[i]
    when 0, nil
      program[immediate_val]
    when 1
      immediate_val
    end
  end
end
