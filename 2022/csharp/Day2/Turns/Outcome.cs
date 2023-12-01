namespace Day2.Turns;

public class Outcome
{
    private Moves Opponent { get; }
    private Score Us { get; }
    public int Scored => (int) GetScore() + (int) Us;

    public Outcome(string[] characters)
    {
        Opponent = Dictionaries.Opponent[characters[0][0]];
        Us = Dictionaries.Desired[characters[1][0]];
    }

    public Moves GetScore()
    {
        return Opponent switch
        {
            Moves.Rock => Us switch
            {
                Score.Lose => Moves.Scissors,
                Score.Draw => Moves.Rock,
                Score.Win => Moves.Paper,
                _ => throw new ArgumentOutOfRangeException()
            },
            Moves.Paper => Us switch
            {
                Score.Lose => Moves.Rock,
                Score.Draw => Moves.Paper,
                Score.Win => Moves.Scissors,
                _ => throw new ArgumentOutOfRangeException()
            },
            Moves.Scissors => Us switch
            {
                Score.Lose => Moves.Paper,
                Score.Draw => Moves.Scissors,
                Score.Win => Moves.Rock,
                _ => throw new ArgumentOutOfRangeException()
            },
            _ => throw new ArgumentOutOfRangeException()
        };
    }
}
