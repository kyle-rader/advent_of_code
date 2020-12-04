using SolverBase;

using System;
using System.Collections;
using System.Collections.Generic;
using System.IO.Abstractions;
using System.Linq;
using System.Text.RegularExpressions;

namespace Solver2020
{
    public class Day4 : Base
    {
        public Day4(IFileSystem fileSystem) : base(fileSystem) { }

        public override string Solve(string inputFile)
        {
            var inputRaw = fileSystem.File.ReadAllText(inputFile);
            var splits = inputRaw.Split(new[] { "\r\n\r\n" }, System.StringSplitOptions.RemoveEmptyEntries);
            return splits.Select(data => new Passport(data)).Where(p => p.Valid()).Count().ToString();
        }

        public override string Solve2(string inputFile)
        {
            var inputRaw = fileSystem.File.ReadAllText(inputFile);
            var splits = inputRaw.Split(new[] { "\r\n\r\n" }, System.StringSplitOptions.RemoveEmptyEntries);
            return splits.Select(data => new Passport(data)).Where(p => p.ValidStrict()).Count().ToString();
        }

        public class Passport
        {
            static readonly Dictionary<string, Func<string, bool>> ReqFields = new Dictionary<string, Func<string, bool>>()
            {
                {"byr", ByrValid },
                {"iyr", IyrValid },
                {"eyr", EyrValid },
                {"hgt", HgtValid },
                {"hcl", HclValid },
                {"ecl", EclValid },
                {"pid" , PidValid },
                // "cid" optional
            };

            private static bool PidValid(string arg)
            {
                return Regex.IsMatch(arg, "[0-9]{9}");
            }

            private static readonly HashSet<string> EyeColors = new HashSet<string>()
            {
                "amb", "blu", "brn", "gry", "grn", "hzl", "oth"
            };

            private static bool EclValid(string arg)
            {
                return EyeColors.Contains(arg);
            }

            private static bool HclValid(string arg)
            {
                return Regex.IsMatch(arg, "#([0-9]|[a-f]){6}");
            }

            private static bool HgtValid(string arg)
            {
                var match = Regex.Match(arg, "([0-9]+)(cm|in)");
                if (!match.Success) return false;
                int.TryParse(match.Groups[1].Value, out int height);

                int min = 59, max = 76;

                if (match.Groups[2].Value == "cm")
                {
                    min = 150;
                    max = 193;
                }
                return height >= min && height <= max;
            }

            private static bool EyrValid(string arg)
            {
                return ValidYear(arg, 2020, 2030);
            }

            private static bool IyrValid(string arg)
            {
                return ValidYear(arg, 2010, 2020);
            }

            private static bool ByrValid(string arg)
            {
                return ValidYear(arg, 1920, 2002);
            }

            private static bool ValidYear(string arg, int min, int max)
            {
                if (int.TryParse(arg, out int expYear))
                    return expYear >= min && expYear <= max;
                return false;
            }

            IDictionary<string, string> fields;

            public Passport(string input)
            {
                fields = new Dictionary<string, string>();

                foreach (string pair in input.Split(new[] { "\r\n", " " }, System.StringSplitOptions.RemoveEmptyEntries))
                {
                    var parts = pair.Split(new[] { ':' }, System.StringSplitOptions.RemoveEmptyEntries);
                    fields[parts[0]] = parts[1];
                }
            }

            public bool Valid()
            {
                return ReqFields.Keys.All(field => fields.ContainsKey(field));
            }

            public bool ValidStrict()
            {
                return ReqFields.All(field => fields.ContainsKey(field.Key) && field.Value(fields[field.Key]));
            }
        }
    }
}
