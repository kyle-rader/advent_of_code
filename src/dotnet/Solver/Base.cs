using System;
using System.Collections.Generic;
using System.IO.Abstractions;
using System.Linq;

namespace Solver
{
    public abstract class Base : ISolver
    {
        public static readonly string[] NewLines = new[] { "\n", "\r\n" };
        public static readonly string[] DoubleNewLines = new[] { "\n\n", "\r\n\r\n" };
        public static readonly string[] Commas = new[] { "," };

        protected readonly IFileSystem fileSystem;
        private readonly string inputFile;

        public Base(IFileSystem fileSystem, string inputFile)
        {
            this.fileSystem = fileSystem;
            this.inputFile = inputFile;
        }

        public abstract double Solve();
        public abstract double Solve2();

        private string _input;
        public string Input
        {
            get
            {
                if (string.IsNullOrEmpty(_input))
                    _input = fileSystem.File.ReadAllText(inputFile);
                return _input;
            }
        }

        public IEnumerable<string> InputLines() => Input.SplitLines();
        public IEnumerable<string> InputLinesByBlankLines() => Input.SplitNoEmpties(DoubleNewLines);
        public IEnumerable<string> InputByCommas() => Input.SplitNoEmpties(Commas);
        public IEnumerable<int> InputInts() => InputLines().Select(int.Parse);
        public IEnumerable<double> InputDoubles() => InputLines().Select(double.Parse);
    }

    public static class StringExtensions
    {
        public static IEnumerable<string> SplitNoEmpties(this string self, string[] delimiters)
        {
            return self.Split(delimiters, StringSplitOptions.RemoveEmptyEntries);
        }

        public static IEnumerable<string> SplitLines(this string self)
        {
            return self.SplitNoEmpties(Base.NewLines);
        }
    }

    public static class IEnumerableExtensions
    {
        public static IEnumerable<int> AsInt(this IEnumerable<string> self)
        {
            return self.Select(int.Parse);
        }

        public static double GeometricAverage(this IEnumerable<int> self)
        {
            double product = self.Aggregate(1.0, (a, i) => a * i);
            return Math.Pow(product, (1.0 / self.Count()));
        }
    }
}
