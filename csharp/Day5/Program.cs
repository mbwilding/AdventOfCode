using Day5;

// Part One
var input = await File.ReadAllLinesAsync("input.txt");

var splitIndex = Array.IndexOf(input, string.Empty);

var stackCountRaw = input[splitIndex - 1];
var stacksRaw = input.Take(splitIndex - 1).ToArray();
var movesRaw = input.Skip(splitIndex + 1).ToArray();

var stackInfos = RegEx.FindStackNumbers(stackCountRaw);
var stackQueues = new List<StackQueue<char>>();
foreach (var stackInfo in stackInfos)
{
    var queue = new StackQueue<char>();
    stackQueues.Add(queue);

    foreach (var stackRaw in stacksRaw)
    {
        if (stackInfo.Index >= stackRaw.Length)
            continue;

        var letter = stackRaw[stackInfo.Index];
        if (char.IsLetter(letter))
        {
            queue.AddBottom(letter);
        }
    }
}

void ProcessMoves(string part, Func<StackQueue<char>, uint, List<char>> func)
{
    var localStackQueue = stackQueues.Select(sq => (StackQueue<char>)sq.Clone()).ToList();
    var moves = RegEx.FindMoveNumbers(movesRaw);
    foreach (var move in moves)
    {
        var queueFrom = localStackQueue[move.From - 1];
        var queueTo = localStackQueue[move.To - 1];

        var stack = func.Invoke(queueFrom, move.Amount);
        queueTo.AddTop(stack);
    }

    var answer = string.Join(string.Empty, localStackQueue.Select(x => x.PeekTop()));
    Console.WriteLine($"Part {part}: {answer}");
}

ProcessMoves("One", (stackQueue, amount) => stackQueue.TakeTop(amount));
ProcessMoves("Two", (stackQueue, amount) => stackQueue.TakeTopRetainOrder(amount));
