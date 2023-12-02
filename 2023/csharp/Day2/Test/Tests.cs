namespace Day2.Tests;

public class UnitTest1
{
    private const int Day = 2;

    [Fact]
    public async Task MockPart1Test() =>
        await Common.Helpers.Test(Day, "mock", Logic.Part1, 8);

    [Fact]
    public async Task MockPart2Test() =>
        await Common.Helpers.Test(Day, "mock", Logic.Part2, 2286);

    [Fact]
    public async Task RealPart1Test() =>
        await Common.Helpers.Test(Day, "real", Logic.Part1, 3059);

    [Fact]
    public async Task RealPart2Test() =>
        await Common.Helpers.Test(Day, "real", Logic.Part2, 65371);
}
