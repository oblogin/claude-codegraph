using System;
using Fx.Statics;

namespace Fx.Properties
{
    public class Settings
    {
        private int _volume = 50;

        public event Action Changed;

        public int Volume
        {
            get { return _volume; }
            set
            {
                _volume = MathUtil.Clamp(value, 0, MathUtil.MaxValue);
                Changed?.Invoke();
            }
        }

        public string PlayerName { get; set; } = "player";

        public bool IsMuted => Volume == 0;
    }
}
