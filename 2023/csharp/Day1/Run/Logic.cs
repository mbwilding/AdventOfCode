using System.Text;

namespace Day1;

public static class Logic
{
    public static int Part1(IEnumerable<string> lines) =>
        lines.AsParallel()
            .Select(line =>
            {
                var digits = line.Where(char.IsDigit)
                    .Select(c => (int)char.GetNumericValue(c))
                    .ToList();

                var first = digits.First();
                var last = digits.Last();

                return first * 10 + last;
            })
            .Sum();

    public static int Part2(IEnumerable<string> lines) =>
        lines.AsParallel()
            .Select(line =>
            {
                int first = 0, last = 0;
                var currentWord = new StringBuilder();

                foreach (var c in line)
                {
                    if (char.IsLetter(c))
                    {
                        currentWord.Append(c);
                        for (int i = 0; i < currentWord.Length; i++)
                        {
                            var currentString = currentWord.ToString(i, currentWord.Length - i);
                            if (!Data.NumberLut.TryGetValue(currentString, out var number))
                            {
                                continue;
                            }

                            if (first == 0)
                            {
                                first = number;
                            }
                            last = number;
                            break;
                        }
                    }
                    else if (char.IsDigit(c))
                    {
                        var digit = (int)char.GetNumericValue(c);
                        if (first == 0)
                        {
                            first = digit;
                        }
                        last = digit;
                        currentWord.Clear();
                    }
                }

                return first * 10 + last;
            })
            .Sum();
}
