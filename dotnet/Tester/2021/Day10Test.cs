using FluentAssertions;

using NUnit.Framework;

using Solver._2021;

namespace Tester._2021
{
    public class Day10Test : TestBase
    {
        [SetUp]
        public void Setup()
        {
            SetInput(@"
");

            solver = new Day10(fileSystem, INPUT_FILE);
        }

        [Test]
        public void Part1()
        {
            Solve1Int().Should().Be(0);
        }

        [Test]
        public void Part2()
        {
            Solve2Int().Should().Be(0);
        }
    }
}
