namespace Day3;

public static class Logic
{
    public static int Part1(List<string> lines)
    {
        return 0; // TODO
    }

    public static int Part2(List<string> lines)
    {
        var rows = GetRowsOfChars(lines);
        return rows
            .SelectMany((row, rowIndex) => row.Select((ch, colIndex) => (ch, rowIndex, colIndex)))
            .Where(x => x.ch == '*')
            .Select(x => GetPartNumbers(rows, x.rowIndex, x.colIndex))
            .Where(partNumbers => partNumbers.Count == 2)
            .Sum(partNumbers => partNumbers.Aggregate(1, (a, b) => a * b));
    }

    private static HashSet<int> GetPartNumbers(char[][] rows, int rowI, int colI) =>
        new(Data.Directions.Select(d => (rowI + d.Item1, colI + d.Item2))
            .Where(pos => IsValidPosition(rows, pos.Item1, pos.Item2) && char.IsDigit(rows[pos.Item1][pos.Item2]))
            .Select(pos => ExtractNumber(rows, pos.Item1, pos.Item2))
            .SelectMany(numberStr => int.TryParse(numberStr, out var num) ? new[] { num } : Array.Empty<int>()));

    private static bool IsValidPosition(char[][] rows, int row, int col) =>
        row >= 0 && row < rows.Length && col >= 0 && col < rows[row].Length;

    private static string ExtractNumber(char[][] rows, int row, int col) =>
        new(rows[row].Skip(rows[row].Take(col).Select((c, index) => new { Char = c, Index = index })
            .LastOrDefault(x => !char.IsDigit(x.Char))?.Index + 1 ?? 0).TakeWhile(char.IsDigit).ToArray());

    private static char[][] GetRowsOfChars(List<string> lines) =>
        lines.Select(line => line.ToCharArray()).ToArray();
}
