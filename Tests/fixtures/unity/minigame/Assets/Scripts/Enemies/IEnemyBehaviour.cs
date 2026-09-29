namespace Game.Enemies
{
    public interface IEnemyBehaviour
    {
        void Begin(Enemy owner);

        // Not a Unity message: a plain interface method called from Enemy.Update.
        void Update(Enemy owner);
    }
}
