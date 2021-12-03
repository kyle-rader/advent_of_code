using System.IO.Abstractions;
using System;
using System.Linq;
using System.Collections;
using System.Collections.Generic;
using System.Text.RegularExpressions;

namespace Solver._2021
{
    public class Day3 : Base
    {
        public Day3(IFileSystem fileSystem, string inputFile) : base(fileSystem, inputFile) { }

        private int BitsToString(IEnumerable<char> bits)
        {
            return Convert.ToInt32(string.Join("", bits), 2);
        }

        private int[] OnesPositionalBitCount(IEnumerable<string> input)
        {
            int n = input.First().Length;
            int[] bitCounts = new int[n];

            foreach (var line in input)
            {
                for (int i = 0; i < line.Length; i++)
                {
                    if (line[i] == '1')
                    {
                        bitCounts[i]++;
                    }
                }
            }
            return bitCounts;
        }

        public override double Solve()
        {
            var input = InputLines();
            int mid = input.Count() / 2;
            int[] bitCounts = OnesPositionalBitCount(input);

            var gamma = BitsToString(bitCounts.Select(c => c >= mid ? '1' : '0'));
            var epsilon = BitsToString(bitCounts.Select(c => c >= mid ? '0' : '1'));
            return gamma * epsilon;
        }

        public override double Solve2()
        {
            var input = InputLines().ToList();
            int co2GeneratorRating = Co2GeneratorRating(input);
            int co2ScrubberRating = Co2ScrubberRating(input);
            return co2GeneratorRating * co2ScrubberRating;
        }

        private int BinaryFilterSearch(List<string> input, Func<bool, char> criterion)
        {
            Dictionary<int, string> set = new Dictionary<int, string>();
            for (int i = 0; i < input.Count(); i++)
                set.Add(i, input[i]);

            int bitPosition = 0;
            int contigencyPlan = 0;

            while (set.Count > 1)
            {
                int[] bitCounts = OnesPositionalBitCount(set.Values);
                char mostCommon = criterion(bitCounts[bitPosition] >= (set.Count - bitCounts[bitPosition]));

                foreach ((int i, string val) in set.Select(item => (item.Key, item.Value)))
                {
                    if (val[bitPosition] != mostCommon)
                    {
                        set.Remove(i);
                    }
                }

                bitPosition++;
                contigencyPlan++;
                if (contigencyPlan > 10_000_000)
                {
                    throw new Exception("Something is probably wrong");
                }
            }
            return Convert.ToInt32(set.First().Value, 2);
        }

        public int Co2GeneratorRating(List<string> input)
        {
            return BinaryFilterSearch(input, (flag => flag ? '1' : '0'));
        }

        public int Co2ScrubberRating(List<string> input)
        {
            return BinaryFilterSearch(input, (flag => flag ? '0' : '1'));
        }
    }
}
