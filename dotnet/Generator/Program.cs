using McMaster.Extensions.CommandLineUtils;

using System;
using System.IO.Abstractions;

namespace Generator
{
    class Program
    {
        static void Main(string[] args)
        {
            IFileSystem fs = new FileSystem();
            IConsole console = PhysicalConsole.Singleton;

            CommandLineApplication app = new CommandLineApplication();
            app.HelpOption("-h|--help");

            var year = app.Argument<int>("year", "Project year").IsRequired();
            var day = app.Argument<int>("day", "Problem day").IsRequired();

            app.OnExecute(() =>
            {
                Generate(fs, console, year.Value, day.Value);
            });

            app.Execute(args);
        }

        private static void Generate(IFileSystem fs, IConsole console, string year, string day)
        {
            string gitRoot;
            try
            {
                gitRoot = Medallion.Shell.Command
                .Run("git", "rev-parse", "--show-toplevel")
                .StandardOutput.ReadToEnd()
                .Trim()
                .Replace('/', fs.Path.DirectorySeparatorChar);
            }
            catch (Exception ex)
            {
                console.Error.WriteLine("Could not find git root directory.");
                console.Error.WriteLine(ex.ToString());
                return;
            }

            var pathSolver = fs.Path.Combine(gitRoot, "dotnet", "Solver", year, $"Day{day}.cs");
            var pathTester = fs.Path.Combine(gitRoot, "dotnet", "Tester", year, $"Day{day}Test.cs");

            bool quit = false;
            if (fs.File.Exists(pathSolver))
            {
                console.Error.WriteLine($"Already exists: {pathSolver}");
                quit = true;
            }
            if (fs.File.Exists(pathTester))
            {
                console.Error.WriteLine($"Already exists: {pathTester}");
                quit = true;
            }
            if (quit) return;

            // Ensure Directories Exist
            foreach (var path in new[] { pathSolver, pathTester })
            {
                var dir = fs.Directory.GetParent(path);
                if (!dir.Exists)
                {
                    console.WriteLine($"Project dir not here yet, creating {dir.FullName}");
                    dir.Create();
                }
            }

            Console.WriteLine($"Generating {pathSolver}");
            fs.File.WriteAllText(pathSolver, $@"using System.IO.Abstractions;
using System;
using System.Linq;
using System.Collections;
using System.Collections.Generic;
using System.Text.RegularExpressions;

namespace Solver._{year}
{{
    public class Day{day} : Base
    {{
        public Day{day}(IFileSystem fileSystem) : base(fileSystem) {{ }}

        public override double Solve(string inputFile)
        {{
            var input = Input(inputFile);
            return null;
        }}

        public override double Solve2(string inputFile)
        {{
            var input = Input(inputFile);
            return null;
        }}
    }}
}}
");
            Console.WriteLine($"Generating {pathTester}");
            fs.File.WriteAllText(pathTester, $@"using FluentAssertions;

using NUnit.Framework;

using Solver._{year};

namespace Tester._{year}
{{
    public class Day{day}Test : TestBase
    {{
        [SetUp]
        public void Setup()
        {{
            SetInput(@""
"");

            solver = new Day{day}(fileSystem);
        }}

        [Test]
        public void Part1()
        {{
            //Solve1Int().Should().Be(0);
        }}

        [Test]
        public void Part2()
        {{
            //Solve2Int().Should().Be(0);
        }}
    }}
}}
");
            console.WriteLine("Done.");
        }
    }
}
