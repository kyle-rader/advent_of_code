using FluentAssertions;

using NUnit.Framework;

using Solver._2020;

namespace Tester._2020
{
    public class Day01Test : TestBase
    {
        [SetUp]
        public void Setup()
        {
            SetInput(@"1721
979
366
299
675
1456");

            solver = new Day01(fileSystem, INPUT_FILE);
        }

        [Test]
        public void Part1Example()
        {
            Solve1Int().Should().Be(514579);
        }

        [Test]
        public void Part2Example()
        {
            Solve2Int().Should().Be(241861950);
        }
    }
}