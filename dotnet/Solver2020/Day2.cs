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
                .Select(item => PasswordIsValid1(item))
                .Where(v => v == true)
                .Count()
                .ToString();
        }

        public override string Solve2(string inputFile)
        {
            return InputItems(inputFile)
                .Select(item => PasswordIsValid2(item))
                .Where(v => v == true)
                .Count()
                .ToString();
        }

        public static bool PasswordIsValid1(string input)
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

        public static bool PasswordIsValid2(string input)
        {
            // 1 - 3 a: abcde
            var parts = input.Split(new[] { '-', ' ', ':' }, StringSplitOptions.RemoveEmptyEntries);
            if (parts.Length != 4) throw new Exception($"Expected 4 parts to be parsed, but got {parts.Length}");

            var opt1 = int.Parse(parts[0]) - 1;
            var opt2 = int.Parse(parts[1]) - 1;
            var target = parts[2].First();
            var password = parts[3];

            return password[opt1] == target ^ password[opt2] == target;
        }
    }
}
