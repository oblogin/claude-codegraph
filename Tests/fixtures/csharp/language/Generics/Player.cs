using System;

namespace Fx.Generics
{
    public class Player : IEntity
    {
        public int Id { get; set; }

        public string Name { get; set; }

        public void Validate()
        {
            if (string.IsNullOrEmpty(Name)) throw new InvalidOperationException("Player needs a name");
        }
    }
}
