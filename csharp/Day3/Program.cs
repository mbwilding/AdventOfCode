using Day3;

// Part One
var input = await File.ReadAllLinesAsync("input.txt");

var rucksacks = input.Select(x => new Rucksack
{
    FirstHalf = x.Take(x.Length / 2).ToArray(),
    SecondHalf = x.Skip(x.Length / 2).ToArray()
}).ToList();

var score = new List<int>();
foreach (var rucksack in rucksacks)
{
    var same = rucksack.FirstHalf
        .Intersect(rucksack.SecondHalf)
        .ToList();

    score.AddRange(same.Select(Logic.Score));
}

var sum = score.Sum();

Console.WriteLine($"Part One: {sum}");

// Part Two
var grouped = input
    .Select((item, index) => new { item, index })
    .GroupBy(x => x.index / 3)
    .Select(g => g
        .Select(x => x.item)
        .ToList())
    .ToList();

var priorities = grouped
    .SelectMany(x => x[0]
        .Intersect(x[1])
        .Intersect(x[2])
        .ToList())
    .ToList();

sum = priorities.Sum(Logic.Score);

Console.WriteLine($"Part Two: {sum}");
