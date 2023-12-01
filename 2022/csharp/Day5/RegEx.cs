using System.Text.RegularExpressions;

namespace Day5;

public static class RegEx
{
    public static List<(ushort Value, int Index)> FindStackNumbers(string input)
    {
        var resultList = new List<(ushort Value, int Index)>();

        var matches = Regex.Matches(input, @"\d+", RegexOptions.Compiled);

        foreach (Match match in matches)
        {
            var value = Convert.ToUInt16(match.Value);
            resultList.Add((value, match.Index));
        }

        return resultList;
    }

    public static List<Move> FindMoveNumbers(string[] inputs)
    {
        var resultList = new List<Move>();

        foreach (var input in inputs)
        {
            var matches = Regex.Matches(input, @"\d+", RegexOptions.Compiled);
            var values = matches.Select(x => Convert.ToUInt16(x.Value)).ToArray();
            resultList.Add(new Move
            {
                Amount = values[0],
                From = values[1],
                To = values[2]
            });
        }

        return resultList;
    }

    public class Move
    {
        public ushort Amount { get; init; }
        public ushort From { get; init; }
        public ushort To { get; init; }
    }
}
