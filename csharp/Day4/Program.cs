// Part One
var input = await File.ReadAllLinesAsync("input.txt");

var split = input
    .Select(x => x.Split(',')
        .Select(y => y.Split('-')
            .Select(int.Parse).ToList()));

var groups = split.Select(x => x.Select(y =>
{
    var start = y[0];
    var end = y[1] - start + 1;
    return Enumerable.Range(start, end);
}).ToList());

var totalOverlap = groups.Select(group =>
{
    var group1 = group[0];
    var group2 = group[1];
    return !group1.Except(group2).Any() || !group2.Except(group1).Any();
});

var totalOverlapped = totalOverlap.Count(x => x);

Console.WriteLine($"Part One: {totalOverlapped}");

// Part Two
var overlap = groups.Select(group =>
{
    var group1 = group[0];
    var group2 = group[1];
    return group1.Intersect(group2).Any();
});

var overlapped = overlap.Count(x => x);

Console.WriteLine($"Part Two: {overlapped}");
