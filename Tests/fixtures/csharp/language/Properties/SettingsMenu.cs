using Fx.Traps;

namespace Fx.Properties
{
    public class SettingsMenu
    {
        private readonly Settings _settings = new Settings();
        private readonly AudioMixer _mixer = new AudioMixer();

        public string Render()
        {
            _settings.Volume = 80;
            int current = _settings.Volume;
            _settings.Volume += 5;
            // Trap: AudioMixer.Volume is a different property with the same name.
            _mixer.Volume = 0.5f;
            string label = nameof(Settings.Volume);
            return label + current + (_settings.IsMuted ? " (muted)" : "");
        }
    }
}
