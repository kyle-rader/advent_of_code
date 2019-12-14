require 'rspec'
require_relative 'day01.rb'

RSpec.describe Day01 do
    describe "#new" do
        it "exists" do
            day01 = Day01.new
            expect(day01).to be
        end
    end
end
