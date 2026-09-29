using System.Collections.Generic;

namespace Fx.Generics
{
    public class Repository<T> where T : IEntity
    {
        private readonly List<T> _items = new List<T>();

        public void Add(T item)
        {
            item.Validate();
            // Trap: List<T>.Add, not a recursive call of Repository<T>.Add.
            _items.Add(item);
        }

        public T Find(int id)
        {
            return _items.Find(x => x.Id == id);
        }
    }
}
