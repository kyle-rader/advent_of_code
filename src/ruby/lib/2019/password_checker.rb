class PasswordChecker
    def self.check(input)
        return false if input.length != 6
        has_double = false
        (1..5).each do |i|
            return false if input[i] < input[i-1]
            has_double |= input[i] == input[i-1]
        end
        has_double
    end

    def self.count_valid(passwords)
        passwords.reduce(0) { |sum, p| sum + (check(p) ? 1 : 0) }
    end

    def self.check_strict(input)
        return false if input.length != 6

        (1..5).each do |i|
            return false if input[i] < input[i-1]
        end

        parts = split_on_parts input
        parts.any? { |part| part.length == 2 }
    end

    def self.count_valid_strict(passwords)
        passwords.reduce(0) { |sum, p| sum + (check_strict(p) ? 1 : 0) }
    end

    def self.split_on_parts(input)
        input[1..-1].each_char.reduce(input[0]) { |chain, c| chain.end_with?(c) ? chain + c : "#{chain}-#{c}" }.split("-")
    end

end
