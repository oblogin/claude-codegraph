using Fx.Events;
using Fx.Extensions;

namespace Fx.Partial
{
    public class Arena
    {
        public int Round(Hero hero, Enemy enemy, Health health)
        {
            int dealt = hero.Dash();
            int taken = enemy.Attack();
            hero.Heal(5);
            CharacterExtensions.Heal(enemy, 1);
            health.Heal(3);
            return dealt - taken;
        }
    }
}
