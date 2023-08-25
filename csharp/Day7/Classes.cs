namespace Day7;


public class Data
{
    public string Name { get; init; }
    public uint? Size { get; init; }
    public Data? Parent { get; init; }
    public List<Data> Children { get; } = new();
    public Type Type { get; init; }

    public long? DirectorySize
    {
        get
        {
            if (Type == Type.File)
            {
                return Size;
            }

            // For directories
            return Children.Sum(child => child.DirectorySize ?? 0);
        }
    }

    public List<Data> FindDirectoriesBySizeRange(long? minSize = null, long? maxSize = null) =>
        FindDirectoriesBySizeRange(this, minSize ?? long.MinValue, maxSize ?? long.MaxValue);

    private List<Data> FindDirectoriesBySizeRange(Data data, long minSize, long maxSize)
    {
        List<Data> directoriesInRange = new List<Data>();

        if (data.Type == Type.Directory && data.DirectorySize >= minSize && data.DirectorySize <= maxSize)
        {
            directoriesInRange.Add(data);
        }

        foreach (var child in data.Children.Where(child => child.Type == Type.Directory))
        {
            directoriesInRange.AddRange(FindDirectoriesBySizeRange(child, minSize, maxSize));
        }

        return directoriesInRange;
    }
}
