using Xunit;

namespace Common;

public static class Helpers
{
    public static async Task Run<T1, T2>(int day, string file, Func<string[], T1> part1, Func<string[], T2> part2)
    {
        var path = $"../../../../../../!data/day{day}/{file}.txt";
        var lines = await File.ReadAllLinesAsync(path).ConfigureAwait(false);

        Console.WriteLine($"Part 1: {part1.Invoke(lines)}");
        Console.WriteLine($"Part 2: {part2.Invoke(lines)}");
    }

    public static async Task Test<T>(int day, string file, Func<string[], T> part, T expected)
    {
        var path = $"../../../../../../!data/day{day}/{file}.txt";
        var lines = await File.ReadAllLinesAsync(path).ConfigureAwait(false);
        var actual = part.Invoke(lines);

        Assert.Equal(expected, actual);
    }
}
