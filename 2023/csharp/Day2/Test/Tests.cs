namespace Day2.Tests;

public class UnitTest1
{
    private const int Day = 2;

    [Fact]
    public async Task Part1Mock() =>
        await Common.Helpers.Test(Day, "mock", Logic.Part1, 8);

    [Fact]
    public async Task Part1Real() =>
        await Common.Helpers.Test(Day, "real", Logic.Part1, 3059);

    [Fact]
    public async Task Part2Mock() =>
        await Common.Helpers.Test(Day, "mock", Logic.Part2, 2286);

    [Fact]
    public async Task Part2Real() =>
        await Common.Helpers.Test(Day, "real", Logic.Part2, 65371);
}
