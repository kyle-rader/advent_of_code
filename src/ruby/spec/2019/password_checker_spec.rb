require 'password_checker'

RSpec.describe PasswordChecker do
    describe "#check" do
        subject { PasswordChecker.check(input) }
        [
            { :input => "1233",   :expected => false },
            { :input => "122345", :expected => true },
            { :input => "111111", :expected => true },
            { :input => "223450", :expected => false },
            { :input => "123789", :expected => false },
        ].each do |arg|
            context "#{arg[:input]} should be #{arg[:expected]}" do
                let(:input) { arg[:input] }
                it { should eq arg[:expected] }
            end
        end

        it "counts valid passwords" do
            passwords = ["1234", "122345", "111111", "223450", "123789" ]
            expect(PasswordChecker.count_valid(passwords)).to eq 2
        end

        it "can find number of valid passwords in range" do
            passwords = (234208..765869).map { |i| i.to_s }
            expect(PasswordChecker.count_valid(passwords)).to eq 1246
        end
    end

    describe "#check_strict" do
        subject { PasswordChecker.check_strict(input) }
        [
            { :input => "122345", :expected => true },
            { :input => "112233", :expected => true },
            { :input => "111122", :expected => true },
            { :input => "1233",   :expected => false },
            { :input => "111111", :expected => false },
            { :input => "223450", :expected => false },
            { :input => "123789", :expected => false },
            { :input => "123444", :expected => false },
        ].each do |arg|
            context "#{arg[:input]} should be #{arg[:expected]}" do
                let(:input) { arg[:input] }
                it { should eq arg[:expected] }
            end
        end

        it "can find number of valid passwords in range" do
            passwords = (234208..765869).map { |i| i.to_s }
            expect(PasswordChecker.count_valid_strict(passwords)).to eq 814
        end
    end

    describe "#split_on_parts" do
        subject { PasswordChecker.split_on_parts input }
        [
            { :input => "122345", :expected => ['1', '22', '3', '4', '5'] },
            { :input => "112233", :expected => ['11', '22', '33'] },
            { :input => "111122", :expected => ['1111', '22'] },
            { :input => "1233",   :expected => ['1', '2', '33'] },
            { :input => "111111", :expected => ['111111'] },
            { :input => "223450", :expected => ['22', '3', '4', '5', '0'] },
            { :input => "123789", :expected => ['1', '2', '3', '7', '8', '9'] },
        ].each do |arg|
            context "#{arg[:input]} should split to #{arg[:expected]}" do
                let(:input) { arg[:input] }
                it { should eq arg[:expected] }
            end
        end
    end
end
