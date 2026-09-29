namespace Fx.Interfaces
{
    public static class StorageUtil
    {
        // Generic method with a constraint: store.Save resolves to IStorage.Save via 'where T : IStorage'.
        public static void SaveAll<T>(T store, string[] items) where T : IStorage
        {
            foreach (string item in items)
            {
                store.Save(item);
            }
        }
    }
}
