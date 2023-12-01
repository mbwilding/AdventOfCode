namespace Day2.Turns;

public class Action
{
    private Moves Opponent { get; }
    private Moves Us { get; }
    public int Scored => (int) GetScore() + (int) Us;

    public Action(string[] characters)
    {
        Opponent = Dictionaries.Opponent[characters[0][0]];
        Us = Dictionaries.Us[characters[1][0]];
    }

    public Score GetScore()
    {
        return Opponent switch
        {
            Moves.Rock => Us switch
            {
                Moves.Rock => Score.Draw,
                Moves.Paper => Score.Win,
                Moves.Scissors => Score.Lose,
                _ => throw new ArgumentOutOfRangeException()
            },
            Moves.Paper => Us switch
            {
                Moves.Rock => Score.Lose,
                Moves.Paper => Score.Draw,
                Moves.Scissors => Score.Win,
                _ => throw new ArgumentOutOfRangeException()
            },
            Moves.Scissors => Us switch
            {
                Moves.Rock => Score.Win,
                Moves.Paper => Score.Lose,
                Moves.Scissors => Score.Draw,
                _ => throw new ArgumentOutOfRangeException()
            },
            _ => throw new ArgumentOutOfRangeException()
        };
    }
}
