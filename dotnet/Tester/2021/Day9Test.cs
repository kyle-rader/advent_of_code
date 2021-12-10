using FluentAssertions;

using NUnit.Framework;

using Solver._2021;

namespace Tester._2021
{
    public class Day9Test : TestBase
    {
        [SetUp]
        public void Setup()
        {
            SetInput(@"2199943210
3987894921
9856789892
8767896789
9899965678
");

            solver = new Day9(fileSystem, INPUT_FILE);
        }

        [Test]
        public void Part1()
        {
            Solve1Int().Should().Be(15);
        }

        [Test]
        public void Part2()
        {
            Solve2Int().Should().Be(0);
        }
    }
}
