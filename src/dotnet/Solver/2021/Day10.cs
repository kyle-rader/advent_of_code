using System.IO.Abstractions;
using System;
using System.Linq;
using System.Collections;
using System.Collections.Generic;
using System.Text.RegularExpressions;
using System.Text;

namespace Solver._2021
{
    public class Day10 : Base
    {
        public class NavSystem
        {
            public enum ParseResultState
            {
                Valid,
                Incomplete,
                Corrupt,
            }

            public struct ParseResult
            {
                public ParseResultState state;
                public double ScoreCorrupt;
                public double ScoreIncomplete;
            }

            public static HashSet<char> Openers = new HashSet<char>()
            {
                '(', '[', '{', '<',
            };

            public static HashSet<char> Closers = new HashSet<char>()
            {
                ')', ']', '}', '>',
            };

            public static readonly Dictionary<char, char> OpenerFromCloser = new Dictionary<char, char>()
            {
                { ')', '(' },
                { ']', '[' },
                { '}', '{' },
                { '>', '<' },
            };

            public static readonly Dictionary<char, char> CloserFromOpener = new Dictionary<char, char>()
            {
                { '(', ')' },
                { '[', ']' },
                { '{', '}' },
                { '<', '>' },
            };

            public static readonly Dictionary<char, double> PointsCorrupt = new Dictionary<char, double>()
            {
                { ')', 3 },
                { ']', 57 },
                { '}', 1197 },
                { '>', 25137 },
            };

            public static readonly Dictionary<char, double> PointsIncomplete = new Dictionary<char, double>()
            {
                { ')', 1 },
                { ']', 2 },
                { '}', 3 },
                { '>', 4 },
            };

            public static ParseResult Validate(string input)
            {
                Stack<char> stack = new Stack<char>();
                foreach (var c in input)
                {
                    var isClosing = Closers.Contains(c);
                    if (Openers.Contains(c))
                    {
                        stack.Push(c);
                    }
                    else if (isClosing && stack.Count == 0)
                    {
                        return new ParseResult()
                        {
                            state = ParseResultState.Corrupt,
                            ScoreCorrupt = PointsCorrupt[c],
                        };
                    }
                    else if (isClosing)
                    {
                        var opener = stack.Pop();
                        if (opener != OpenerFromCloser[c])
                        {
                            return new ParseResult()
                            {
                                state = ParseResultState.Corrupt,
                                ScoreCorrupt = PointsCorrupt[c],
                            };
                        }
                    }
                    else
                    {
                        throw new ArgumentException($"Unkown instruction char found '{c}'");
                    }
                }

                var result = new ParseResult()
                {
                    state = ParseResultState.Valid,
                };

                if (stack.Count > 0)
                {
                    result.state = ParseResultState.Incomplete;
                    while (stack.Count > 0)
                    {
                        result.ScoreIncomplete = (result.ScoreIncomplete * 5) + PointsIncomplete[CloserFromOpener[stack.Pop()]];
                    }
                }
                return result;
            }
        }

        public Day10(IFileSystem fileSystem, string inputFile) : base(fileSystem, inputFile) { }

        public override double Solve()
        {
            var input = InputLines();
            return input.Aggregate(0.0, (acc, line) =>
            {
                var result = NavSystem.Validate(line);
                return acc + result.ScoreCorrupt;
            });
        }

        public override double Solve2()
        {
            var input = InputLines();
            var incompleteScores = input
                .Select(NavSystem.Validate)
                .Where(r => r.state == NavSystem.ParseResultState.Incomplete)
                .Select(r => r.ScoreIncomplete)
                .ToList();
            incompleteScores.Sort();
            return incompleteScores[incompleteScores.Count / 2];
        }
    }
}
