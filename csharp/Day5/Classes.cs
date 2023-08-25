namespace Day5;

public class StackQueue<T> : ICloneable
{
    private readonly LinkedList<T> _linkedList = new();

    public int Count => _linkedList.Count;

    public void AddBottom(T obj)
    {
        _linkedList.AddLast(obj);
    }

    public void AddTop(T obj)
    {
        _linkedList.AddFirst(obj);
    }

    public T TakeTop()
    {
        var obj = PeekTop();
        _linkedList.RemoveFirst();
        return obj;
    }

    public T TakeBottom()
    {
        var obj = PeekBottom();
        _linkedList.RemoveLast();
        return obj;
    }

    public T PeekTop()
    {
        return _linkedList.First!.Value;
    }

    public T PeekBottom()
    {
        return _linkedList.Last!.Value;
    }

    public void AddTop(List<T> objs)
    {
        foreach (var obj in objs)
        {
            AddTop(obj);
        }
    }

    public List<T> TakeTop(uint amount)
    {
        var list = new List<T>();

        for (uint i = 0; i < amount; i++)
        {
            list.Add(TakeTop());
        }

        return list;
    }

    public List<T> TakeTopRetainOrder(uint amount)
    {
        var list = TakeTop(amount);

        list.Reverse();

        return list;
    }

    public void AddBottom(List<T> objs)
    {
        foreach (var obj in objs)
        {
            AddBottom(obj);
        }
    }

    public List<T> TakeBottom(uint amount)
    {
        var list = new List<T>();

        for (uint i = 0; i < amount; i++)
        {
            list.Add(TakeBottom());
        }

        return list;
    }

    public List<T> TakeBottomRetainOrder(uint amount)
    {
        var list = TakeBottom(amount);

        list.Reverse();

        return list;
    }

    public object Clone()
    {
        var clonedStackQueue = new StackQueue<T>();

        foreach (var item in _linkedList)
        {
            clonedStackQueue.AddBottom(item);
        }

        return clonedStackQueue;
    }
}
