using FluentAssertions;

using NUnit.Framework;

using Solver._2015;

namespace Tester._2015
{
    public class Day02Test : TestBase
    {
        [SetUp]
        public void Setup()
        {
            SetInput(@"2x3x4
");

            solver = new Day02(fileSystem, INPUT_FILE);
        }

        [Test]
        public void Box()
        {
            var subject = new Day02.Box(new[] { 2, 3, 4 });
            subject.WrappingPaper.Should().Be(58);
        }

        [Test]
        public void Part1()
        {
            Solve1Int().Should().Be(58);
        }

        [Test]
        public void Part2()
        {
            Solve2Int().Should().Be(34);
        }
    }
}
