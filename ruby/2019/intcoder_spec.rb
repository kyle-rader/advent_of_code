require_relative 'intcoder.rb'

RSpec.describe Intcoder do
    subject {
        Intcoder.new program
    }

    describe "#run" do
        context "with the sample program" do
            let(:program) { "1,9,10,3,2,3,11,0,99,30,40,50" }
            it "gets expected output value" do
                expect(subject.run).to eq 3500
            end
        end
    end
end