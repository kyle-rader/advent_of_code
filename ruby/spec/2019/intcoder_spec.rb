require 'intcoder'

RSpec.describe Intcoder, "#run" do
  let(:input) { [] }
  let(:program) { nil }
  let(:intcoder) {
      Intcoder.new(program)
  }
  let(:final_program) { intcoder.program }
  let(:program_0) { final_program[0] }
  subject { intcoder.output }

  before(:each) do
    input.each { |val| intcoder.in(val) }
    intcoder.run
  end

  let(:day5_program) { "3,225,1,225,6,6,1100,1,238,225,104,0,2,218,57,224,101,-3828,224,224,4,224,102,8,223,223,1001,224,2,224,1,223,224,223,1102,26,25,224,1001,224,-650,224,4,224,1002,223,8,223,101,7,224,224,1,223,224,223,1102,44,37,225,1102,51,26,225,1102,70,94,225,1002,188,7,224,1001,224,-70,224,4,224,1002,223,8,223,1001,224,1,224,1,223,224,223,1101,86,70,225,1101,80,25,224,101,-105,224,224,4,224,102,8,223,223,101,1,224,224,1,224,223,223,101,6,91,224,1001,224,-92,224,4,224,102,8,223,223,101,6,224,224,1,224,223,223,1102,61,60,225,1001,139,81,224,101,-142,224,224,4,224,102,8,223,223,101,1,224,224,1,223,224,223,102,40,65,224,1001,224,-2800,224,4,224,1002,223,8,223,1001,224,3,224,1,224,223,223,1102,72,10,225,1101,71,21,225,1,62,192,224,1001,224,-47,224,4,224,1002,223,8,223,101,7,224,224,1,224,223,223,1101,76,87,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,108,226,677,224,102,2,223,223,1005,224,329,1001,223,1,223,1107,677,226,224,102,2,223,223,1006,224,344,1001,223,1,223,7,226,677,224,1002,223,2,223,1005,224,359,101,1,223,223,1007,226,226,224,102,2,223,223,1005,224,374,101,1,223,223,108,677,677,224,102,2,223,223,1006,224,389,1001,223,1,223,107,677,226,224,102,2,223,223,1006,224,404,101,1,223,223,1108,677,226,224,102,2,223,223,1006,224,419,1001,223,1,223,1107,677,677,224,1002,223,2,223,1006,224,434,101,1,223,223,1007,677,677,224,102,2,223,223,1006,224,449,1001,223,1,223,1108,226,677,224,1002,223,2,223,1006,224,464,101,1,223,223,7,677,226,224,102,2,223,223,1006,224,479,101,1,223,223,1008,226,226,224,102,2,223,223,1006,224,494,101,1,223,223,1008,226,677,224,1002,223,2,223,1005,224,509,1001,223,1,223,1007,677,226,224,102,2,223,223,1005,224,524,1001,223,1,223,8,226,226,224,102,2,223,223,1006,224,539,101,1,223,223,1108,226,226,224,1002,223,2,223,1006,224,554,101,1,223,223,107,226,226,224,1002,223,2,223,1005,224,569,1001,223,1,223,7,226,226,224,102,2,223,223,1005,224,584,101,1,223,223,1008,677,677,224,1002,223,2,223,1006,224,599,1001,223,1,223,8,226,677,224,1002,223,2,223,1006,224,614,1001,223,1,223,108,226,226,224,1002,223,2,223,1006,224,629,101,1,223,223,107,677,677,224,102,2,223,223,1005,224,644,1001,223,1,223,8,677,226,224,1002,223,2,223,1005,224,659,1001,223,1,223,1107,226,677,224,102,2,223,223,1005,224,674,1001,223,1,223,4,223,99,226" }

  context "with the sample program" do
    let(:program) { "1,9,10,3,2,3,11,0,99,30,40,50" }
    subject { program_0 }
    it { is_expected.to eq 3500 }
  end

  context "with a full program" do
    let(:program) { "1,50,64,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,10,19,2,6,19,23,1,23,5,27,1,27,13,31,2,6,31,35,1,5,35,39,1,39,10,43,2,6,43,47,1,47,5,51,1,51,9,55,2,55,6,59,1,59,10,63,2,63,9,67,1,67,5,71,1,71,5,75,2,75,6,79,1,5,79,83,1,10,83,87,2,13,87,91,1,10,91,95,2,13,95,99,1,99,9,103,1,5,103,107,1,107,10,111,1,111,5,115,1,115,6,119,1,119,10,123,1,123,10,127,2,127,13,131,1,13,131,135,1,135,10,139,2,139,6,143,1,143,9,147,2,147,6,151,1,5,151,155,1,9,155,159,2,159,6,163,1,163,2,167,1,10,167,0,99,2,14,0,0" }
    subject { program_0 }
    it { is_expected.to eq 19690720 }
  end

  context "with input and output" do
    let(:program) { "3,0,4,0,99" }
    let(:input) { [45] }
    it { is_expected.to eq [45] }
  end

  context "with immediate arg modes" do
    let(:program) { "1101,100,-1,4,0" }
    subject { final_program }
    it { is_expected.to eq [1101, 100, -1, 4, 99] }
  end

  describe "opcode 8: equals" do
    [
      "3,3,1108,-1,8,3,4,3,99",
      "3,9,8,9,10,9,4,9,99,-1,8"
    ].each do |p|
      context "with program #{p}" do
        let(:program) { p }
        context "is equal" do
          let(:input) { [8] }
          it { is_expected.to eq [1] }
        end
        context "is not equal" do
          let(:input) { [5] }
          it { is_expected.to eq [0] }
        end
      end
    end
  end

  describe "opcode 7: less than" do
    [
      "3,3,1107,-1,8,3,4,3,99",
      "3,9,7,9,10,9,4,9,99,-1,8"
    ].each do |p|
      context "with program #{p}" do
        let(:program) { p }
        context "is less than" do
          let(:input) { [5] }
          it { is_expected.to eq [1] }
        end
        context "is not less than" do
          let(:input) { [24] }
          it { is_expected.to eq [0] }
        end
        context "is equal" do
          let(:input) { [8] }
          it { is_expected.to eq [0] }
        end
      end
    end
  end
end

RSpec.describe Intcoder, "self.get_modes" do
  subject { Intcoder.get_modes m }
  [
    { :m => 123, :exp => [3, 2, 1] },
    { :m => 11, :exp => [1, 1] },
    { :m => 1, :exp => [1] },
    { :m => 0, :exp => [] },
  ].each do |args|
    context "when m = #{args[:m]}" do
      let(:m) { args[:m] }
      it { is_expected.to eq args[:exp] }
    end
  end
end
