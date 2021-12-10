using System.IO.Abstractions;
using System;
using System.Linq;
using System.Collections;
using System.Collections.Generic;
using System.Text.RegularExpressions;
using Solver.Helpers;

namespace Solver._2021
{
    public class Day8 : Base
    {
        public Day8(IFileSystem fileSystem, string inputFile) : base(fileSystem, inputFile) { }

        public static readonly Dictionary<HashSet<char>, int> SevenSegDecoder = new Dictionary<HashSet<char>, int>()
        {
            { new HashSet<char>() { 'a', 'b', 'c',      'e', 'f', 'g' }, 0 },
            { new HashSet<char>() {           'c',           'f',     }, 1 },
            { new HashSet<char>() { 'a',      'c', 'd', 'e',      'g' }, 2 },
            { new HashSet<char>() { 'a',      'c', 'd',      'f', 'g' }, 3 },
            { new HashSet<char>() {      'b', 'c', 'd',      'f',     }, 4 },
            { new HashSet<char>() { 'a', 'b',      'd',      'f', 'g' }, 5 },
            { new HashSet<char>() { 'a', 'b',      'd', 'e', 'f', 'g' }, 6 },
            { new HashSet<char>() { 'a',      'c',           'f',     }, 7 },
            { new HashSet<char>() { 'a', 'b', 'c', 'd', 'e', 'f', 'g' }, 8 },
            { new HashSet<char>() { 'a', 'b', 'c', 'd',      'f', 'g' }, 9 },
        };

        public class Decoder
        {
            Dictionary<HashSet<char>, char> map = new Dictionary<char, char>();
            private HashSet<char>[] codex;
            private HashSet<char>[] fives;
            private HashSet<char>[] sixes;

            public Decoder(string input)
            {
                var parts = input.Split();
                Array.Sort(parts, (a, b) => a.Length.CompareTo(b.Length));
                codex = parts.Select(p => Sets.From(p)).ToArray();

                fives = codex.Where(i => i.Count == 5).ToArray();
                sixes = codex.Where(i => i.Count == 6).ToArray();

                var one = parts[0].AsSet();   // 2 chars
                var seven = parts[1].AsSet(); // 3 chars
                var four = parts[2].AsSet();  // 4 chars
                var eight = parts[9].AsSet(); // 7 chars

                // We know 3 - 1 gives us position a:
                var reveal_a = Remove(seven, one);
                map[reveal_a.First()] = 'a';
            }

            internal double Decode(string v)
            {
                throw new NotImplementedException();
            }
        }

        public static HashSet<T> Remove<T>(HashSet<T> a, HashSet<T> b)
        {
            var newA = Sets.From(a);
            foreach (T item in b)
            {
                newA.Remove(item);
            }
            return newA;
        }

        public override double Solve()
        {
            var input = InputLines();
            int count = 0;
            HashSet<int> uniqueSignals = new HashSet<int>() { 2, 4, 3, 7 };
            foreach (var line in input)
            {
                var parts = line.SplitNoEmpties(new[] { "|" }).ToList();
                foreach (var digit in parts[1].Split())
                {
                    var signals = new HashSet<char>(digit.ToCharArray()).Count;
                    if (uniqueSignals.Contains(signals))
                    {
                        count++;
                    }
                }
            }
            return count;
        }

        public override double Solve2()
        {
            var input = InputLines();
            double sum = 0;
            HashSet<int> uniqueSignals = new HashSet<int>() { 2, 4, 3, 7 };
            foreach (var line in input)
            {
                var parts = line.SplitNoEmpties(new[] { "|" }).ToList();
                Decoder decoder = new Decoder(parts[0]);
                sum += decoder.Decode(parts[1]);
            }
            return sum;
        }
    }
}
