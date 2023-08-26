using Day8;

var input = await File.ReadAllLinesAsync("input.txt");
var forest = new Forest(input);

Console.WriteLine($"Part One: {forest.CountVisibleTrees()}");
Console.WriteLine($"Part Two: {forest.CalculateMaxScenicScore()}");
