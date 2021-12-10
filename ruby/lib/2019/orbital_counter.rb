class OrbitalCounter
  attr_reader :map

  def initialize(orbital_map)
    @map = Hash.new
    orbital_map.split().each do |orbit|
      center, orbiter = orbit.split(")")
      @map[orbiter] = center
    end
  end

  def count
    cnt = 0
    @map.each_key do |k|
      cnt += count_from(k)
    end
    cnt
  end

  def count_from(key)
    return 1 if map[key] == "COM"
    1 + count_from(map[key])
  end
end
