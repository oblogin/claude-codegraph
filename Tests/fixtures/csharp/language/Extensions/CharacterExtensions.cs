using Fx.Partial;

namespace Fx.Extensions
{
    public static class CharacterExtensions
    {
        // Extension method on the base class: applies to Hero and Enemy.
        public static void Heal(this Character character, int amount)
        {
            character.Hp += amount;
        }
    }
}
