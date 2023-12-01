namespace Day2;

public static class Dictionaries
{
    public static readonly Dictionary<char, Moves> Opponent = new()
    {
        {'A', Moves.Rock},
        {'B', Moves.Paper},
        {'C', Moves.Scissors}
    };

    public static readonly Dictionary<char, Moves> Us = new()
    {
        {'X', Moves.Rock},
        {'Y', Moves.Paper},
        {'Z', Moves.Scissors}
    };

    public static readonly Dictionary<char, Score> Desired = new()
    {
        {'X', Score.Lose},
        {'Y', Score.Draw},
        {'Z', Score.Win}
    };
}
