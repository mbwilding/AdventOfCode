namespace Day1.Tests;

public class UnitTest1
{
    private const int Day = 1;

    [Fact]
    public async Task Part1Mock() =>
        await Common.Helpers.Test(Day, "mock1", Logic.Part1, 142);

    [Fact]
    public async Task Part1Real() =>
        await Common.Helpers.Test(Day, "real", Logic.Part1, 54573);

    [Fact]
    public async Task Part2Mock() =>
        await Common.Helpers.Test(Day, "mock2", Logic.Part2, 281);

    [Fact]
    public async Task Part2Real() =>
        await Common.Helpers.Test(Day, "real", Logic.Part2, 54591);
}
