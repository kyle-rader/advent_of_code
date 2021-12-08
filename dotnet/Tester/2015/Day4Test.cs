using FluentAssertions;

using NUnit.Framework;

using Solver._2015;

namespace Tester._2015
{
    public class Day4Test : TestBase
    {
        [SetUp]
        public void Setup()
        {
            SetInput(@"iwrupvqb");

            solver = new Day4(fileSystem, INPUT_FILE);
        }

        [Test]
        [Ignore("This takes a few seconds")]
        public void Part1()
        {
            Solve1Int().Should().Be(346386);
        }

        [Test]
        [Ignore("This takes a few minutes")]
        public void Part2()
        {
            Solve2Int().Should().Be(9958218);
        }
    }
}
