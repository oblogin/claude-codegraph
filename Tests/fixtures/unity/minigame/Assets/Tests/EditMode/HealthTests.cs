using Game.Player;
using NUnit.Framework;
using UnityEngine;

namespace Game.Tests
{
    public class HealthTests
    {
        private static Health CreateHealth(int maxHp)
        {
            var go = new GameObject("health-test");
            var health = go.AddComponent<Health>();
            health.Init(maxHp);
            return health;
        }

        [Test]
        public void TakeDamage_ReducesCurrent()
        {
            var health = CreateHealth(10);
            health.TakeDamage(3);
            Assert.AreEqual(7, health.Current);
        }

        [Test]
        public void TakeDamage_Lethal_InvokesOnDied()
        {
            var health = CreateHealth(2);
            bool died = false;
            health.OnDied.AddListener(() => died = true);
            health.TakeDamage(5);
            Assert.IsTrue(died);
        }

        [Test]
        public void Heal_ClampsToMax()
        {
            var health = CreateHealth(10);
            health.Heal(5);
            Assert.AreEqual(10, health.Current);
        }
    }
}
