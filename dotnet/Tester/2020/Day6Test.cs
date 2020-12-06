using FluentAssertions;

using NUnit.Framework;

using Solver._2020;

namespace Tester._2020
{
    public class Day6Test : TestBase
    {
        [SetUp]
        public void Setup()
        {
            SetInput(@"abc

a
b
c

ab
ac

a
a
a
a

b");

            solver = new Day6(fileSystem);
        }

        [Test]
        public void Part1()
        {
            Solve1Int().Should().Be(11);
        }

        [Test]
        public void Part2()
        {
            Solve2Int().Should().Be(6);
        }
    }
}
