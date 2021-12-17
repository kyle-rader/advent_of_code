using FluentAssertions;

using NUnit.Framework;

using Solver._2021;
using NavSystem = Solver._2021.Day10.NavSystem;

namespace Tester._2021
{
    public class Day10Test : TestBase
    {
        [SetUp]
        public void Setup()
        {
            SetInput(@"
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
");

            solver = new Day10(fileSystem, INPUT_FILE);
        }

        [TestCase("()")]
        [TestCase("[]")]
        [TestCase("{}")]
        [TestCase("<>")]
        [TestCase("(<>)")]
        [TestCase("[(<>)]")]
        [TestCase("{[(<>)()<>]}")]
        [TestCase("[][](<>)")]
        public void NavSystem_Validate_Valid(string input)
        {
            NavSystem.Validate(input).state.Should().Be(NavSystem.ParseResultState.Valid);
        }

        [TestCase("(")]
        [TestCase("[")]
        [TestCase("{")]
        [TestCase("<")]
        [TestCase("({[")]
        [TestCase("({[]")]
        [TestCase("({[]}")]
        [TestCase("({[]})(")]
        public void NavSystem_Validate_Incomplete(string input)
        {
            NavSystem.Validate(input).state.Should().Be(NavSystem.ParseResultState.Incomplete);
        }

        [TestCase("(]")]
        [TestCase("[)")]
        [TestCase("{>")]
        [TestCase("<}")]
        [TestCase("([]>")]
        public void NavSystem_Validate_Corrupt(string input)
        {
            NavSystem.Validate(input).state.Should().Be(NavSystem.ParseResultState.Corrupt);
        }

        [TestCase("({}))")]
        public void NavSystem_Validate_TooManyClosers(string input)
        {
            NavSystem.Validate(input).state.Should().Be(NavSystem.ParseResultState.Corrupt);
        }

        [Test]
        public void Part1()
        {
            Solve1Double().Should().Be(26397);
        }

        [Test]
        public void Part2()
        {
            Solve2Double().Should().Be(288957);
        }
    }
}
