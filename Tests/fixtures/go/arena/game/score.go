package game

import "example.com/arena/events"

// Score tracks points.
type Score struct {
	points    int
	snapshots int
}

// Add adds points.
func (s *Score) Add(n int) {
	s.points += n
}

// Points returns the current score.
func (s *Score) Points() int {
	return s.points
}

// OnMonsterKilled awards points for a kill.
func (s *Score) OnMonsterKilled(e events.Event) {
	s.Add(10)
}
