namespace Day1.Tests;

public class UnitTest1
{
    private const int Day = 1;

    [Fact]
    public async Task MockPart1Test() =>
        await Common.Helpers.Test(Day, "mock1", Logic.Part1, 142);

    [Fact]
    public async Task MockPart2Test() =>
        await Common.Helpers.Test(Day, "mock2", Logic.Part2, 281);

    [Fact]
    public async Task RealPart1Test() =>
        await Common.Helpers.Test(Day, "real", Logic.Part1, 54573);

    [Fact]
    public async Task RealPart2Test() =>
        await Common.Helpers.Test(Day, "real", Logic.Part2, 54591);
}
