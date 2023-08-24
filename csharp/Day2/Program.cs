using Day2.Turns;
using Action = Day2.Turns.Action;

// Part One
var input = await File.ReadAllLinesAsync("input.txt");

var split = input
    .Select(x => x.Split(' '))
    .ToList();

var turnsPartOne = split
    .Select(x => new Action(x))
    .ToList();

var sum = turnsPartOne
    .Select(x => x.Scored)
    .Sum();

Console.WriteLine($"Part One: {sum}");

// Part Two
var turnsPartTwo = split
    .Select(x => new Outcome(x))
    .ToList();

sum = turnsPartTwo
    .Select(x => x.Scored)
    .Sum();

Console.WriteLine($"Part Two: {sum}");
