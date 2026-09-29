using UnityEngine;
using UnityEngine.SceneManagement;

namespace Game.UI
{
    public class MainMenu : MonoBehaviour
    {
        [SerializeField] private GameObject _gameOverPanel;

        // Called by Button.onClick of PlayButton in Assets/Scenes/Main.unity; no C# callers.
        public void OnPlayClicked()
        {
            SceneManager.LoadScene("Bonus");
        }

        // Called by Health.OnDied, configured as a prefab override in Assets/Scenes/Main.unity.
        public void ShowGameOver()
        {
            _gameOverPanel.SetActive(true);
        }
    }
}
