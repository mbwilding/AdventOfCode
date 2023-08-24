namespace Day3;

public static class Logic
{
    private const int Uppercase = 'A' - 27;
    private const int Lowercase = 'a' - 1;

    public static int Score(char character)
    {
        if (char.IsUpper(character))
        {
            return character - Uppercase;
        }
        if (char.IsLower(character))
        {
            return character - Lowercase;
        }

        throw new ArgumentOutOfRangeException();
    }
}
