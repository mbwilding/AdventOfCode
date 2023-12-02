namespace Day2;

public static class Logic
{
    public static int Part1(IEnumerable<string> lines) =>
        lines
            .AsParallel()
            .Select(line =>
            {
                var parts = line.Split(": ");
                var gameId = int.Parse(parts[0].Split().Last());

                var isValid = parts[1]
                    .Split("; ")
                    .All(set => set.Split(", ")
                        .All(c =>
                        {
                            var vec = c.Split();
                            var count = int.Parse(vec[0]);
                            var color = vec[1];
                            return count <= Data.MaxCubesLut[color];
                        }));

                return isValid ? gameId : 0;
            })
            .Sum();

    public static int Part2(IEnumerable<string> lines) =>
        lines
            .AsParallel()
            .Select(line => line.Split(": ")[1]
                .Split("; ")
                .SelectMany(set => set.Split(", "))
                .Select(cube =>
                {
                    var parts = cube.Split();
                    return (Color: parts[1], Count: int.Parse(parts[0]));
                })
                .GroupBy(c => c.Color)
                .Select(g => g.Max(c => c.Count))
                .Aggregate(1, (product, count) => product * count))
            .Sum();
}
