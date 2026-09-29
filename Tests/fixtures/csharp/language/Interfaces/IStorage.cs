namespace Fx.Interfaces
{
    public interface IStorage
    {
        void Save(string data);

        string Load();
    }
}
