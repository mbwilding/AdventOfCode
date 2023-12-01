// Part One
var input = await File.ReadAllTextAsync("../../../../2022/!data/day1/real.txt");
var lineEnding = input.Contains("\r\n") ? "\r\n" : "\n"; // Git changed the line endings

var elvesCalories = input
    .Split(lineEnding + lineEnding)
    .Select(elfSection => elfSection.Split(lineEnding)
        .Where(line => !string.IsNullOrEmpty(line))
        .Select(uint.Parse)
        .Sum(x => x))
    .ToList();

var max = elvesCalories.Max();

Console.WriteLine($"Part 1: {max}");

// Part Two
var top3Calories = elvesCalories
    .OrderByDescending(x => x)
    .Take(3)
    .Sum();

Console.WriteLine($"Part 2: {top3Calories}");
