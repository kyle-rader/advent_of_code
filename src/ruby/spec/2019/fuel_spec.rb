require 'fuel.rb'

RSpec.describe Fuel do
    let(:input) {
        %w[137139 104321 137149 82531 97698 56831 115133 64329 111730 145953 73388 57230 61935 58542 147631 79366 115484 86997 80362 129109 58568 121969 63696 68116 86668 62059 59485 132507 107823 94467 53032 140962 129499 80599 147570 96463 126169 108575 133312 146929 79826 114449 110908 66107 62171 91677 128188 90483 81045 96006 110366 140765 148360 54565 56664 121547 78839 123739 115408 123245 92419 132564 80022 103370 145366 145211 110360 145897 140817 77978 138064 148134 86208 89472 67117 63423 148536 105835 107783 98601 66702 50459 55127 79808 79628 76264 134392 125547 118186 80947 121669 107315 145093 56296 117226 105409 149238 142651 103286 139215]
    }

    subject { Fuel }

    describe "#Fuel" do
        describe "#for_mass" do
            [
                [12, 2],
                [14, 2],
                [1969, 654],
                [100756, 33583]
            ].each do |mass, expected_fuel|
                context "Given Module (#{mass})" do
                    it "Returns fuel needed (#{expected_fuel})" do
                        expect(subject.for_mass(mass)).to eq expected_fuel
                    end
                end
            end
        end

        describe "#for_masses" do
            let(:masses) { %w(12 14 1969) }
            it "returns the sum of the fuels needed" do
                expect(subject.for_masses(masses)).to eq 658
            end
        end

        describe "#for_mass_with_fuel" do
            [
                [2, 0],
                [14, 2],
                [1969, 966],
                [100756, 50346],
            ].each do |mass, expected_fuel|
                context "Given Mass (#{mass})" do
                    it "Returns fuel needed (#{expected_fuel})" do
                        expect(subject.for_mass_with_fuel(mass)).to eq expected_fuel
                    end
                end
            end
        end

        describe "#for_masses_with_fuel" do
            let(:masses) { %w(2 14 1969 100756) }
            it "gets the total sum" do
                expect(subject.for_masses_with_fuel(masses)).to eq (50346 + 966 + 2)
            end
        end
    end
end
