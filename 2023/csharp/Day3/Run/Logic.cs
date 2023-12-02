namespace Day3;

public static class Logic
{
    public static int Part1(IEnumerable<string> lines) =>
        lines
            .AsParallel()
            .Select(line =>
            {
                return 0;
            })
            .Sum();

    public static int Part2(IEnumerable<string> lines) =>
        lines
            .AsParallel()
            .Select(line =>
            {
                return 0;
            })
            .Sum();
}
