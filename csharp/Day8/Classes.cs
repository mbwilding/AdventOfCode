namespace Day8;

public class Tree
{
    public int X { get; }
    public int Y { get; }
    public int Height { get; }

    public Tree(int x, int y, int height)
    {
        X = x;
        Y = y;
        Height = height;
    }
}

public class Forest
{
    private const char CharOffset = '0';
    private readonly List<Tree> _trees = new();
    private readonly int _size;

    public Forest(string[] input)
    {
        _size = input.Length;

        for (int x = 0; x < _size; x++)
        for (int y = 0; y < _size; y++)
        {
            int height = input[x][y] - CharOffset;
            _trees.Add(new Tree(x, y, height));
        }
    }

    private (bool Visible, int Count) CheckVisibilityAndDistanceFromDirection(Tree tree, int dx, int dy)
    {
        int distance = 0;
        int x = tree.X + dx;
        int y = tree.Y + dy;

        while (x >= 0 && x < _size && y >= 0 && y < _size)
        {
            var other = _trees.Find(t => t.X == x && t.Y == y);
            distance++;

            if (other != null && other.Height >= tree.Height)
                return (false, distance);

            x += dx;
            y += dy;
        }

        return (true, distance);
    }

    // Part One

    public int CountVisibleTrees()
    {
        int visibleCount = 0;
        foreach (var tree in _trees)
        {
            // Check if the tree is on the edge.
            if (tree.X == 0 || tree.X == _size - 1 || tree.Y == 0 || tree.Y == _size - 1)
            {
                visibleCount++;
                continue;
            }

            if (CheckVisibilityAndDistanceFromDirection(tree, -1, 0).Visible ||
                CheckVisibilityAndDistanceFromDirection(tree, 1, 0).Visible ||
                CheckVisibilityAndDistanceFromDirection(tree, 0, -1).Visible ||
                CheckVisibilityAndDistanceFromDirection(tree, 0, 1).Visible)
            {
                visibleCount++;
            }
        }

        return visibleCount;
    }

    // Part Two

    public int CalculateMaxScenicScore()
    {
        int maxScenicScore = 0;

        foreach (var tree in _trees)
        {
            int up = CheckVisibilityAndDistanceFromDirection(tree, -1, 0).Count;
            int down = CheckVisibilityAndDistanceFromDirection(tree, 1, 0).Count;
            int left = CheckVisibilityAndDistanceFromDirection(tree, 0, -1).Count;
            int right = CheckVisibilityAndDistanceFromDirection(tree, 0, 1).Count;
            int scenicScore = up * down * left * right;
            maxScenicScore = Math.Max(maxScenicScore, scenicScore);
        }

        return maxScenicScore;
    }
}
