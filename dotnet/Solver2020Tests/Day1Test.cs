using FluentAssertions;

using NUnit.Framework;

using Solver2020;

namespace Solver2020Tests
{
    public class Day1Test : TestBase
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

            solver = new Day1(fileSystem);
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