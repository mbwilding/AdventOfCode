var input = await File.ReadAllLinesAsync("input.txt");
var buffer = input.First();

void ProcessMoves(string part, uint consecutive)
{
    var queue = new Queue<char>();
    for (int i = 0; i < buffer.Length; i++)
    {
        if (queue.Count >= consecutive)
        {
            queue.Dequeue();
        }
        queue.Enqueue(buffer[i]);

        if (queue.Distinct().Count() == consecutive)
        {
            Console.WriteLine($"Part {part}: {i + 1}");
            break;
        }
    }
}

ProcessMoves("One", 4);
ProcessMoves("Two", 14);
