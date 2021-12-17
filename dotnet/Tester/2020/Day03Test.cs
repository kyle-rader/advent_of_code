using FluentAssertions;

using NUnit.Framework;

using Solver._2020;

namespace Tester._2020
{
    public class Day03Test : TestBase
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

            solver = new Day03(fileSystem, INPUT_FILE);
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
