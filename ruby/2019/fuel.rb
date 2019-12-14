class Fuel
    def self.for_mass(mass)
        (mass/3).to_i - 2
    end

    def self.for_masses(masses)
        masses.map { |m|
            for_mass(m.to_i)
        }.sum
    end

    def self.for_mass_with_fuel(mass)
        fuel = for_mass(mass)
        (fuel < 0 ? 0 : fuel) + (fuel <= 0 ? 0 : for_mass_with_fuel(fuel))
    end
end