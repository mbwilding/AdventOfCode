using Common;
using Day7;
using Type = Day7.Type;

// Part One
var input = await File.ReadAllLinesAsync("input.txt");
var stackQueue = new StackQueue<string>(input);

Data fileSystem = new Data
{
    Name = "/",
    Type = Type.Directory
};

var workingDirectory = fileSystem;
while (stackQueue.Any())
{
    var fullCommand = stackQueue.TakeTop();
    if (fullCommand.StartsWith('$'))
    {
        var commands = fullCommand[2..].Split(' ');
        var command = commands[0];
        var argument = commands.Length > 1 ? commands[1] : null;
        switch (command)
        {
            case "ls":
                while (stackQueue.Any() && !stackQueue.PeekTop().StartsWith("$"))
                {
                    var entity = stackQueue.TakeTop();
                    var split = entity.Split(' ');
                    var lsCommand = split[0];
                    var lsArgument = split.Length > 1 ? split[1] : null;
                    switch (lsCommand)
                    {
                        case "dir":
                            workingDirectory.Children.Add(new Data
                            {
                                Name = lsArgument!,
                                Parent = workingDirectory,
                                Type = Type.Directory
                            });
                            break;
                        default:
                            workingDirectory.Children.Add(new Data
                            {
                                Name = lsArgument!,
                                Size = uint.Parse(lsCommand),
                                Type = Type.File
                            });
                            break;
                    }
                }
                break;
            case "cd":
                switch (argument)
                {
                    case "/":
                        workingDirectory = fileSystem;
                        break;
                    case "..":
                        workingDirectory = workingDirectory.Parent;
                        break;
                    default:
                        workingDirectory = workingDirectory.Children.Single(x => x.Name == argument);
                        break;
                }
                break;
        }
    }
}

var partOne = fileSystem
    .FindDirectoriesBySizeRange(maxSize: 100_000)
    .Sum(x => x.DirectorySize);

Console.WriteLine($"Part One: {partOne}");

// Part Two
const int totalSpace = 70_000_000;
const int requiredSpace = 30_000_000;

var spaceUsed = fileSystem.DirectorySize!.Value;
var spaceAvailable = totalSpace - spaceUsed;
var minRequired = requiredSpace - spaceAvailable;

var partTwo = fileSystem
    .FindDirectoriesBySizeRange(minSize: minRequired)
    .MinBy(x => x.DirectorySize)?
    .DirectorySize;
Console.WriteLine($"Part Two: {partTwo}");
