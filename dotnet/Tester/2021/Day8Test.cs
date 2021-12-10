using FluentAssertions;

using NUnit.Framework;

using Solver._2021;

namespace Tester._2021
{
    /*
       0:      1:      2:      3:      4:
     aaaa    ....    aaaa    aaaa    ....
    b    c  .    c  .    c  .    c  b    c
    b    c  .    c  .    c  .    c  b    c
     ....    ....    dddd    dddd    dddd
    e    f  .    f  e    .  .    f  .    f
    e    f  .    f  e    .  .    f  .    f
     gggg    ....    gggg    gggg    ....
       6       2       5       5       4
================================================
================================================

      5:      6:      7:      8:      9:
     aaaa    aaaa    aaaa    aaaa    aaaa
    b    .  b    .  .    c  b    c  b    c
    b    .  b    .  .    c  b    c  b    c
     dddd    dddd    ....    dddd    dddd
    .    f  e    f  .    f  e    f  .    f
    .    f  e    f  .    f  e    f  .    f
     gggg    gggg    ....    gggg    gggg
      5       6       3       7        6

    a = 
    b = c,f
    c = b,d
    d = a
    e = c,f
    f = 
    g = b,d
    
    be - 1
    edb - 7 *
    cgeb - 4 *
    cdefg
    fecdb
    fabcd
    = common in len(5): cfd => adg
    cbdgef
    fgaecd
    agebfd
    cfbegad - 8
    */
    public class Day8Test : TestBase
    {
        [SetUp]
        public void Setup()
        {
            SetInput(@"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
");

            solver = new Day8(fileSystem, INPUT_FILE);
        }

        [TestCase("", 42)]
        public void TestCaseOne(string input, int expected)
        {

        }

        [Test]
        public void Part1()
        {
            Solve1Int().Should().Be(26);
        }

        [Test]
        public void Part2()
        {
            //Solve2Int().Should().Be(0);

            var key1 = Day8.Set("abc");
            Day8.SevenSegDecoder[]
        }
    }
}
