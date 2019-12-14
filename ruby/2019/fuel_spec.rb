require 'rspec'
require_relative 'fuel.rb'

RSpec.describe Fuel do
    subject { Fuel }

    describe "#Fuel" do
        [
            [12, 2],
            [14, 2],
            [1969, 654],
            [100756, 33583]
        ].each do |module_size, expected_fuel|
            context "Given Module (#{module_size})" do
                it "Returns fuel needed (#{expected_fuel})" do
                    expect(subject.for_module(module_size)).to eq expected_fuel
                end
            end
        end

        context "with an array of modules" do
            let(:mods) { %w(12 14 1969) }

            it "returns the sum of the fuels needed" do
                expect(subject.for_modules(mods)).to eq 658
            end
        end
    end
end
