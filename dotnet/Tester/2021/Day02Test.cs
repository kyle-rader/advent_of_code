using FluentAssertions;

using NUnit.Framework;

using Solver._2021;

namespace Tester._2021
{
    public class Day02Test : TestBase
    {
        [SetUp]
        public void Setup()
        {
            SetInput(@"forward 5
down 5
forward 8
up 3
down 8
forward 2
");

            solver = new Day02(fileSystem, INPUT_FILE);
        }

        [Test]
        public void Part1()
        {
            Solve1Int().Should().Be(150);
        }

        [Test]
        public void Part2()
        {
            Solve2Int().Should().Be(900);
        }
    }
}
