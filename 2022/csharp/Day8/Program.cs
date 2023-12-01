using Day8;

var input = await File.ReadAllLinesAsync("../../../../../!data/day8/real.txt");
var forest = new Forest(input);

Console.WriteLine($"Part 1: {forest.CountVisibleTrees()}");
Console.WriteLine($"Part 2: {forest.CalculateMaxScenicScore()}");
