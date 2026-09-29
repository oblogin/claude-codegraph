package game_test

import (
	"testing"

	"example.com/arena/game"
)

type fakeEntity struct {
	id      int
	updates int
}

func (f *fakeEntity) ID() int { return f.id }

func (f *fakeEntity) Update(dt float64) { f.updates++ }

func TestTickUpdatesEntities(t *testing.T) {
	w := game.NewWorld()
	f := &fakeEntity{id: 1}
	w.Add(f)
	w.Tick(0.1)
	if f.updates != 1 {
		t.Fatalf("updates = %d, want 1", f.updates)
	}
}
