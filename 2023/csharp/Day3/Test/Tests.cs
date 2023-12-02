namespace Day3.Tests;

public class UnitTest1
{
    private const int Day = 3;

    [Fact]
    public async Task MockPart1Test() =>
        await Common.Helpers.Test(Day, "mock", Logic.Part1, 0);

    [Fact]
    public async Task MockPart2Test() =>
        await Common.Helpers.Test(Day, "mock", Logic.Part2, 0);

    [Fact]
    public async Task RealPart1Test() =>
        await Common.Helpers.Test(Day, "real", Logic.Part1, 0);

    [Fact]
    public async Task RealPart2Test() =>
        await Common.Helpers.Test(Day, "real", Logic.Part2, 0);
}
