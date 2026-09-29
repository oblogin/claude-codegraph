using UnityEngine;

namespace Game.UI
{
    public class Tutorial : MonoBehaviour
    {
        [SerializeField] private GameObject _panel;

        // Same name as MainMenu.OnPlayClicked; called by the TutorialButton UnityEvent.
        public void OnPlayClicked()
        {
            _panel.SetActive(true);
        }
    }
}
