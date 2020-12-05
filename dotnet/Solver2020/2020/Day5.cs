using System;
using System.Collections.Generic;
using System.IO.Abstractions;
using System.Linq;

namespace Solver
{
    public class Day5 : Base
    {
        public Day5(IFileSystem fileSystem) : base(fileSystem)
        {
        }

        public override string Solve(string inputFile)
        {
            return InputItemsStrings(inputFile).Select(x => new BoardingPass(x).Id).Max().ToString();
        }

        public override string Solve2(string inputFile)
        {
            var ids = InputItemsStrings(inputFile).Select(x => new BoardingPass(x).Id);
            return FindBoardingPass(ids).ToString();
        }

        public class BoardingPass
        {
            public int Row;
            public int Col;
            public int Id;

            public BoardingPass(string input)
            {
                int min = 0, max = 127;
                for (int i = 0; i < 6; i++)
                {
                    if (input[i] == 'F')
                        max = max - (max - min) / 2;
                    else if (input[i] == 'B')
                        min = min + (max - min) / 2;
                }
                Row = BinarySearch(input.Substring(0, 7), 'F', 'B', 0, 127);
                Col = BinarySearch(input.Substring(7), 'L', 'R', 0, 7);
                Id = 8 * Row + Col;
            }

            public static int BinarySearch(string input, char low, char high, int min, int max)
            {
                var n = input.Length;
                for (int i = 0; i < n - 1; i++)
                {
                    if (input[i] == low)
                        max = max - 1 - (max - min) / 2;
                    else if (input[i] == high)
                        min = min + (int)Math.Ceiling((max - min) / 2.0);
                }
                return input[n - 1] == low ? min : max;
            }
        }

        public static int FindBoardingPass(IEnumerable<int> ids)
        {
            HashSet<int> targets = new HashSet<int>();
            foreach (int i in ids)
            {
                // First check if we have seen something with +/- 1 of this id before.
                if (targets.Contains(i-1))
                {
                    return i - 1;
                }
                else if (targets.Contains(i+1))
                {
                    return i + 1;
                }

                targets.Add(i + 1);
                targets.Add(i - 1);
            }
            throw new Exception("Did not find a target id!");
        }
    }
}
