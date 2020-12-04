using SolverBase;

using System.Collections;
using System.Collections.Generic;
using System.IO.Abstractions;
using System.Linq;

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
            return null;
        }

        public class Passport
        {
            static readonly HashSet<string> RequiredFields = new HashSet<string>()
            {
                "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", // "cid" optional
            };

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
                return RequiredFields.All(field => fields.ContainsKey(field));
            }
        }
    }
}
