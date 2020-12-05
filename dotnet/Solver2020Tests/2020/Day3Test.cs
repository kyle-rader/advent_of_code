using FluentAssertions;

using NUnit.Framework;

using Solver;

namespace Tester
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
            Solve2Double().Should().Be(336);
        }
    }
}
