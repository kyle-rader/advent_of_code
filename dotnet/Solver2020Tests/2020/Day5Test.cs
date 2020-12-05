using FluentAssertions;

using NUnit.Framework;

using Solver;

namespace Tester
{
    public class Day5Test : TestBase
    {
        [SetUp]
        public void Setup()
        {
            SetInput(@"FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL");
            solver = new Day5(fileSystem);
        }

        [Test]
        public void Part1Example()
        {
            Solve1Int().Should().Be(820);
        }

        [Test]
        public void Part2ExampleBadPassports()
        {
            SetInput(@"");
            Solve2Int().Should().Be(0);
        }

        [TestCase("FBFBBFFRLR", 44, 5, 357)]
        [TestCase("BFFFBBFRRR", 70, 7, 567)]
        [TestCase("FFFBBBFRRR", 14, 7, 119)]
        [TestCase("BBFFBBFRLL", 102, 4, 820)]
        [TestCase("FFFFFFFRRR", 0, 7, 7)]
        [TestCase("FFFFFFFLLL", 0, 0, 0)]
        public void BoardingPassCanBeParsed(string input, int row, int col, int id)
        {
            var subject = new Day5.BoardingPass(input);
            subject.Row.Should().Be(row);
            subject.Col.Should().Be(col);
            subject.Id.Should().Be(id);
        }
    }
}
