namespace Fx.Interfaces
{
    public class MemoryStore : IStorage
    {
        private string _data = "";

        public void Save(string data)
        {
            _data = data;
        }

        public string Load()
        {
            return _data;
        }
    }
}
