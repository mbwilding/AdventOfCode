namespace Day3.Tests;

public class UnitTest1
{
    private const int Day = 3;

    // [Fact]
    // public async Task Part1Mock() =>
    //     await Common.Helpers.Test(Day, "mock", Logic.Part1, 4361);

    // [Fact]
    // public async Task Part1Real() =>
    //     await Common.Helpers.Test(Day, "real", Logic.Part1, 530849);

    [Fact]
    public async Task Part2Mock() =>
        await Common.Helpers.Test(Day, "mock", Logic.Part2, 467835);

    [Fact]
    public async Task Part2Real() =>
        await Common.Helpers.Test(Day, "real", Logic.Part2, 84900879);
}
