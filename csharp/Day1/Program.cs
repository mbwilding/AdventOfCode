// Part One
var input = await File.ReadAllTextAsync("input.txt");

var elvesCalories = input
    .Split("\n\n")
    .Select(elfSection => elfSection.Split('\n')
        .Where(line => !string.IsNullOrEmpty(line))
        .Select(uint.Parse)
        .Sum(x => x))
    .ToList();

var max = elvesCalories.Max();

Console.WriteLine($"Part One: {max}");

// Part Two
var top3Calories = elvesCalories
    .OrderByDescending(x => x)
    .Take(3)
    .Sum();

Console.WriteLine($"Part Two: {top3Calories}");
