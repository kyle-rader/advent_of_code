using SolverBase;

using System;
using System.Collections.Generic;
using System.IO.Abstractions;
using System.Linq;

namespace Solver2020
{
    public class Day2 : Base, ISolver
    {
        public Day2(IFileSystem fileSystem) : base(fileSystem) { }

        public override string Solve(string inputFile)
        {
            return InputItems(inputFile)
                .Select(item => PasswordIsValid(item))
                .Where(v => v == true)
                .Count()
                .ToString();
        }

        public override string Solve2(string inputFile)
        {
            throw new NotImplementedException();
        }

        public static bool PasswordIsValid(string input)
        {
            // 1 - 3 a: abcde
            var parts = input.Split(new[] { '-', ' ', ':' }, StringSplitOptions.RemoveEmptyEntries);
            if (parts.Length != 4) throw new Exception($"Expected 4 parts to be parsed, but got {parts.Length}");

            var min = int.Parse(parts[0]);
            var max = int.Parse(parts[1]);
            var target = parts[2].First();
            var password = parts[3];

            var freq = new Dictionary<char, int>();
            foreach (char c in password)
            {
                if (freq.ContainsKey(c))
                    freq[c] = freq[c] + 1;
                else
                    freq[c] = 1;
            }

            freq.TryGetValue(target, out int targetCount);
            return targetCount >= min && targetCount <= max;
        }
    }
}
