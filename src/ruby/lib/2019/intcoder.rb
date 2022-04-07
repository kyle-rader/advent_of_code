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
    :opcode, :arg_mode, :verbose

  def initialize(program, verbose = false)
    @program = program
      .split(",")
      .map {|c| c.strip.to_i }
    @verbose = verbose
    @opcode_ptr = 0
    @input = []
    @output = []
    puts "Created Intcoder in verbose mode with program:\n#{program}" if verbose
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
    puts "Clearing caches" if verbose
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
    puts "step: by #{step_by} (#{opcode_ptr} -> #{opcode_ptr + step_by})" if verbose
    @opcode_ptr += step_by
  end

  def process
    puts "process opcode: #{opcode}" if verbose
    case opcode
    when 1
      do_add
    when 2
      do_mult
    when 3
      do_input
    when 4
      do_output
    when 5
      do_jump_if_true
    when 6
      do_jump_if_false
    when 7
      do_less_than
    when 8
      do_equals
    else
      raise "Oops, we don't know how to handle opcode '#{opcode}'"
    end
  end

  def set_step(n)
    puts "set_step: #{n}" if verbose
    @step_by = n
  end

  def do_add
    puts "do_add: #{arg(0)} + #{arg(1)}" if verbose
    set_val(arg(0) + arg(1))
    set_step 4
  end

  def do_mult
    puts "do_mult: #{arg(0)} * #{arg(1)}" if verbose
    set_val(arg(0) * arg(1))
    set_step 4
  end

  def do_input
    val = input.pop
    puts "do_input: store #{val} at index #{program[opcode_ptr + 1]}" if verbose
    @program[program[opcode_ptr + 1]] = val
    set_step 2
  end

  def do_output
    puts "do_output: #{arg(0)}" if verbose
    @output << arg(0)
    set_step 2
  end

  def do_jump_if_true
    puts "do_jump_if_true: #{arg(0)} != 0" if verbose
    jump_if { arg(0) != 0 }
  end

  def do_jump_if_false
    puts "do_jump_if_false: #{arg(0)} == 0" if verbose
    jump_if { arg(0) == 0 }
  end

  def jump_if(&block)
    if block.call
      puts "  true: opcode_ptr = #{arg(1)}" if verbose
      @opcode_ptr = arg(1)
      set_step 0
    else
      puts "  false" if verbose
      set_step 3
    end
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
    puts "set_val: store #{val} at index #{program[opcode_ptr + 3]}" if verbose
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
