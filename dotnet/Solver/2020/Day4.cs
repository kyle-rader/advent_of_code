using System;
using System.Collections.Generic;
using System.IO.Abstractions;
using System.Linq;
using System.Text.RegularExpressions;

namespace Solver._2020
{
    public class Day4 : Base
    {
        public Day4(IFileSystem fileSystem) : base(fileSystem) { }

        public override double Solve(string inputFile)
        {
            var inputRaw = fileSystem.File.ReadAllText(inputFile);
            var splits = inputRaw.Split(new[] { "\r\n\r\n" }, StringSplitOptions.RemoveEmptyEntries);
            return splits
                .Select(data => new Passport(data))
                .Where(p => p.Valid())
                .Count();
        }

        public override double Solve2(string inputFile)
        {
            var inputRaw = fileSystem.File.ReadAllText(inputFile);
            var splits = inputRaw.Split(new[] { "\r\n\r\n" }, StringSplitOptions.RemoveEmptyEntries);

            var passports = splits.Select(data => new Passport(data)).ToArray();

            var invalid = passports.Where(p => !p.ValidStrict()).Count();
            var valid = passports.Where(p => p.ValidStrict()).Count();

            return valid;
        }

        public class Passport
        {
            private static readonly HashSet<string> EyeColors = new HashSet<string>()
            {
                "amb", "blu", "brn", "gry", "grn", "hzl", "oth"
            };

            private static readonly Dictionary<string, Func<string, bool>> ReqFields = new Dictionary<string, Func<string, bool>>()
            {
                {"byr", (arg) => Within(arg, 1920, 2002) },
                {"iyr", (arg) => Within(arg, 2010, 2020) },
                {"eyr", (arg) => Within(arg, 2020, 2030) },
                {"hgt", HgtValid },
                {"hcl", (arg) => Regex.IsMatch(arg.ToLower(), "^#([0-9a-f]){6}$") },
                {"ecl", (arg) => EyeColors.Contains(arg) },
                {"pid" , (arg) => Regex.IsMatch(arg, "^[0-9]{9}$") },
            };

            private static bool HgtValid(string arg)
            {
                if (arg.EndsWith("in"))
                {
                    return Within(arg.Substring(0, arg.Length - 2), 59, 76);
                }
                else if (arg.EndsWith("cm"))
                {
                    return Within(arg.Substring(0, arg.Length - 2), 150, 193);
                }
                else
                {
                    return false;
                }
            }

            private static bool Within(string arg, int min, int max)
            {
                if (int.TryParse(arg, out int expYear))
                    return expYear >= min && expYear <= max;
                return false;
            }

            IDictionary<string, string> fields;

            public Passport(string input)
            {
                fields = new Dictionary<string, string>();

                foreach (string pair in input.Split(new[] { "\r\n", " " }, StringSplitOptions.RemoveEmptyEntries))
                {
                    var parts = pair.Split(new[] { ':' }, StringSplitOptions.RemoveEmptyEntries);
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
