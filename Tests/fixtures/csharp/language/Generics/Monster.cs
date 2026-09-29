using System;

namespace Fx.Generics
{
    public class Monster : IEntity
    {
        public int Id { get; set; }

        public int Level { get; set; }

        public void Validate()
        {
            if (Level < 1) throw new InvalidOperationException("Monster level must be positive");
        }
    }
}
