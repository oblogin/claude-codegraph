using System.IO;

namespace Fx.Interfaces
{
    public class FileStore : IStorage, IBackup
    {
        private readonly string _path;

        public FileStore(string path)
        {
            _path = path;
        }

        // Implicit implementation of IStorage.Save only: IBackup.Save is implemented explicitly below.
        public void Save(string data)
        {
            File.WriteAllText(_path, data);
        }

        public string Load()
        {
            return File.ReadAllText(_path);
        }

        // Explicit implementation: reachable only through an IBackup reference.
        void IBackup.Save(string data)
        {
            File.WriteAllText(_path + ".bak", data);
        }
    }
}
