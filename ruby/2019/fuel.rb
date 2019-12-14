class Fuel
    def self.for_module(module_size)
        (module_size/3).to_i - 2
    end

    def self.for_modules(module_sizes)
        module_sizes.map { |m|
            for_module(m.to_i)
        }.sum
    end
end