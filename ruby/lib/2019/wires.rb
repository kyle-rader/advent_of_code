class Wires
    attr_accessor :wires
    def initialize(input)
        @wires = input
            .split("\n")
            .select { |line| line.length > 0 }
            .map { |line| line.split(",") }
    end

    def nearest_cross_distance
        wire1 = Set.new
        follow_wire(wires[0]) { |x,y| wire1.add([x,y]) }
        min_dist = nil

        follow_wire(wires[1]) do |x,y|
            dist = x.abs + y.abs
            min_dist = dist if wire1.include?([x,y]) && ((min_dist == nil) || dist < min_dist)
        end
        min_dist
    end

    def min_steps_to_cross
        wire1 = Hash.new
        steps = 0
        follow_wire(wires[0]) do |x,y|
            steps += 1
            key = "#{x},#{y}"
            wire1[key] = steps unless wire1.include? key
        end
        steps = 0
        min_sum = nil
        follow_wire(wires[1]) do |x,y|
            steps += 1
            key = "#{x},#{y}"
            if wire1.include? key
                sum = wire1[key] + steps
                if min_sum == nil || sum < min_sum
                    min_sum = sum
                end
            end
        end
        min_sum
    end

    def follow_wire(wire)
        x = y = 0
        wire.each do |step|
            dir, dist = parse_step(step)
            dist.times do
                case dir
                when 'U'
                    y += 1
                when 'D'
                    y -= 1
                when 'R'
                    x += 1
                when 'L'
                    x -= 1
                end
                yield x, y
            end
        end
    end

    def parse_step(step)
        return step[0], step[1..-1].to_i
    end
end
