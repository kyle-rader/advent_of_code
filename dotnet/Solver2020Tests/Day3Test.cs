using FluentAssertions;

using NUnit.Framework;

using Solver2020;

namespace Solver2020Tests
{
    public class Day3Test : TestBase
    {
        [SetUp]
        public void Setup()
        {
            SetInput(@"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#");

            solver = new Day3(fileSystem);
        }

        [Test]
        public void Part1Example()
        {
            Solve1Int().Should().Be(7);
        }

        [Test]
        public void Part2Example()
        {
            //Solve2Int().Should().Be(0);
        }
    }
}
