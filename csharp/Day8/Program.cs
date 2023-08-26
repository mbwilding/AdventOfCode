using Day8;

var input = await File.ReadAllLinesAsync("input.txt");
var forest = new Forest(input);

int visibleCount = forest.CountVisibleTrees();
Console.WriteLine($"Part One: {visibleCount}");

int maxScenicScore = forest.CalculateMaxScenicScore();
Console.WriteLine($"Part Two: {maxScenicScore}");
