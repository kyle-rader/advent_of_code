using System.IO.Abstractions;
using System;
using System.Linq;
using System.Collections;
using System.Collections.Generic;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace Solver._2021
{
    public class Day4 : Base
    {
        public Day4(IFileSystem fileSystem, string inputFile) : base(fileSystem, inputFile) { }


        public class BingoBoard
        {
            public int Score;
            public int n; // board size
            public int[] rowCounts;
            public int[] colCounts;
            public Dictionary<int, (int, int)> unCalled;

            public BingoBoard(string input)
            {
                // Get n - board size
                var lines = input.SplitLines().ToList();
                n = lines.Count;

                // init tracking vars
                rowCounts = new int[n];
                colCounts = new int[n];

                unCalled = new Dictionary<int, (int, int)>();

                for (int i = 0; i < n; i++)
                {
                    var row = lines[i]
                        .SplitNoEmpties(new[] { " " })
                        .Select(int.Parse).ToList();
                    
                    if (n != row.Count)
                        throw new ArgumentException($"Row has unexpected count of items! n = {n} but row has {row.Count} items.");

                    for (int j = 0; j < n; j++)
                    {
                        int v = row[j];
                        unCalled.Add(v, (i, j));
                    }
                }
            }

            public bool Call(int v)
            {
                if (!unCalled.ContainsKey(v)) return false;

                (int row, int col) = unCalled[v];
                rowCounts[row]++;
                colCounts[col]++;

                unCalled.Remove(v);

                if (rowCounts[row] == n || colCounts[col] == n)
                {
                    // win
                    Score = unCalled.Keys.Sum() * v;
                    return true;
                }

                return false;
            }
        }

        public override double Solve()
        {
            var input = InputLinesByBlankLines();
            var numbers = input.First().SplitNoEmpties(new[] { "," }).Select(int.Parse);
            var boards = input.Skip(1).Select(b => new BingoBoard(b)).ToList();

            foreach (var i in numbers)
            {
                foreach (var b in boards)
                {
                    if (b.Call(i)) return b.Score;
                }
            }
            return -1;
        }

        public override double Solve2()
        {
            var input = InputLinesByBlankLines();
            var numbers = input.First().SplitNoEmpties(new[] { "," }).Select(int.Parse);
            var boards = input.Skip(1).Select(b => new BingoBoard(b)).ToList();
            var winningBoards = new HashSet<int>();

            foreach (var number in numbers)
            {
                for (int i = 0; i < boards.Count; i++)
                {
                    if (winningBoards.Contains(i)) continue;

                    if (boards[i].Call(number))
                    {
                        if (winningBoards.Count >= boards.Count-1)
                        {
                            // found last winning board
                            return boards[i].Score;
                        }
                        winningBoards.Add(i);
                    }
                }
            }
            return -1;
        }
    }
}
