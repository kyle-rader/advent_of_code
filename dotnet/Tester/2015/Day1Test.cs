using FluentAssertions;

using NUnit.Framework;

using Solver._2015;

namespace Tester._2015
{
    public class Day1Test : TestBase
    {
        [SetUp]
        public void Setup()
        {
            SetInput(@"
");

            solver = new Day1(fileSystem);
        }

        [TestCase("(())", 0)]
        [TestCase("(())((", 2)]
        public void Part1(string input, int expected)
        {
            SetInput(input);
            Solve1Int().Should().Be(expected);
        }

        [TestCase("(()))", 5)]
        [TestCase("(())()))(()", 7)]
        public void Part2(string input, int expected)
        {
            SetInput(input);
            Solve2Int().Should().Be(expected);
        }
    }
}
