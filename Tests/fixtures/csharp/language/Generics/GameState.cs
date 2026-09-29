using Fx.Traps;

namespace Fx.Generics
{
    public class GameState
    {
        private readonly Repository<Player> _players = new Repository<Player>();
        private readonly Repository<Monster> _monsters = new Repository<Monster>();

        public void AddPlayer(Player player)
        {
            _players.Add(player);
        }

        public void AddMonster(Monster monster)
        {
            _monsters.Add(monster);
        }

        public bool Submit(Form form)
        {
            // Trap: Form.Validate() is not IEntity.Validate().
            return form.Validate();
        }
    }
}
