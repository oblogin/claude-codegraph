using NUnit.Framework;

namespace Game.Core.Tests
{
    public class StatsTests
    {
        [Test]
        public void Add_ReturnsSum()
        {
            Assert.AreEqual(5, Stats.Add(2, 3));
        }

        [Test]
        public void Info_DoesNotThrow()
        {
            Assert.DoesNotThrow(() => Log.Info("test"));
        }
    }
}
