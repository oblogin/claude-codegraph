namespace Fx.Interfaces
{
    public class StorageClient
    {
        private readonly IStorage _storage;

        public StorageClient(IStorage storage)
        {
            _storage = storage;
        }

        public void Persist(string data)
        {
            _storage.Save(data);
        }

        public void Backup(IBackup backup, string data)
        {
            backup.Save(data);
        }

        public void Direct(FileStore fileStore)
        {
            fileStore.Save("direct");
        }
    }
}
