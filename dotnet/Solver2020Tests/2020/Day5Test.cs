using FluentAssertions;

using NUnit.Framework;

using Solver;

using System.Collections.Generic;

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
        public void Part1FindLargestSeatId()
        {
            Solve1Int().Should().Be(820);
        }

        [Test]
        public void Part2FindYourSeatId()
        {
            SetInput(@"");
            Solve2Int().Should().Be(0);
        }

        [TestCase(new[] { 0, 1, 4, 6, 200 }, 5)]
        [TestCase(new[] { 0, 1, 4, 200, 250, 15, 198, 300 }, 199)]
        [TestCase(new[] { 0, 2 }, 1)]
        public void FindMissingBoardingPass(IEnumerable<int> ids, int expectedId)
        {
            Day5.FindBoardingPass(ids).Should().Be(expectedId);
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
