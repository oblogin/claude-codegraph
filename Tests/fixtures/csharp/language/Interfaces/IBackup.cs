namespace Fx.Interfaces
{
    // Same member signature as IStorage.Save: FileStore implements this one explicitly.
    public interface IBackup
    {
        void Save(string data);
    }
}
