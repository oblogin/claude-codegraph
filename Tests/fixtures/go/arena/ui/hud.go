package ui

import (
	"fmt"

	"example.com/arena/events"
)

// HUD shows the score and the clock.
type HUD struct {
	Score   int
	elapsed float64
}

// Update advances the HUD clock.
func (h *HUD) Update(dt float64) {
	h.elapsed += dt
}

// Line formats the HUD line.
func (h *HUD) Line() string {
	return fmt.Sprintf("score %d, time %.1f", h.Score, h.elapsed)
}

// OnMonsterKilled prints kills to the console.
func OnMonsterKilled(e events.Event) {
	fmt.Println("hud: killed", e.Payload)
}
