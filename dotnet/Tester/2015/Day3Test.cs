using FluentAssertions;

using NUnit.Framework;

using Solver._2015;

namespace Tester._2015
{
    public class Day3Test : TestBase
    {
        [SetUp]
        public void Setup()
        {
            SetInput(@"^v^v^v^v^v>");
            solver = new Day3(fileSystem, INPUT_FILE);
        }

        [TestCase("^v^v^v^v^v", 2)]
        [TestCase("^v^v^v^v^v>", 3)]
        [TestCase("^v^v><><<", 4)]
        public void Part1(string input, int expected)
        {
            SetInput(input);
            Solve1Int().Should().Be(expected);
        }

        [Test]
        public void Part2()
        {
            Solve2Int().Should().Be(12);
        }
    }
}
