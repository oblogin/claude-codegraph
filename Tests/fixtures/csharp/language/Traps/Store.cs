namespace Fx.Traps
{
    // Trap: same simple name as the alias 'Store' in Aliases/AliasUser.cs.
    public class Store
    {
        public int Count { get; private set; }

        public void Save(string data)
        {
            Count++;
        }
    }
}
