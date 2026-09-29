// Package game wires entities, rendering and events together.
package game

import (
	"sync"

	"example.com/arena/entity"
	"example.com/arena/events"
	"example.com/arena/render"
)

// Actor is an entity that can also be drawn.
type Actor interface {
	entity.Entity
	render.Drawer
}

// SpawnRequest asks the world to create a monster.
type SpawnRequest struct {
	Kind string
}

// World owns all game objects.
type World struct {
	mu       sync.Mutex
	entities []entity.Entity
	actors   []Actor
	Canvas   *render.Canvas
	Bus      *events.Bus
	score    *Score
	jobs     chan func()
}

// NewWorld creates a world and starts its job runner.
func NewWorld() *World {
	w := &World{
		Canvas: &render.Canvas{},
		Bus:    events.NewBus(),
		score:  &Score{},
		jobs:   make(chan func(), 16),
	}
	w.Bus.Subscribe(events.TopicKilled, w.score.OnMonsterKilled)
	go w.runJobs()
	return w
}

// Add registers an entity.
func (w *World) Add(e entity.Entity) {
	w.mu.Lock()
	defer w.mu.Unlock()
	w.entities = append(w.entities, e)
	if a, ok := e.(Actor); ok {
		w.actors = append(w.actors, a)
	}
}

// Tick updates every entity and draws every actor.
func (w *World) Tick(dt float64) {
	w.mu.Lock()
	defer w.mu.Unlock()
	for _, e := range w.entities {
		e.Update(dt)
	}
	drawers := make([]render.Drawer, 0, len(w.actors))
	for _, a := range w.actors {
		drawers = append(drawers, a)
	}
	w.Canvas.DrawAll(drawers)
}

// Kill publishes a kill event and schedules a snapshot.
func (w *World) Kill(m *entity.Monster) {
	w.Bus.Publish(events.TopicKilled, m.Kind)
	w.Enqueue(w.saveSnapshot)
}

// StartSpawner consumes spawn requests in a goroutine.
func (w *World) StartSpawner(requests <-chan SpawnRequest) {
	go w.spawnLoop(requests)
}

func (w *World) spawnLoop(requests <-chan SpawnRequest) {
	for req := range requests {
		w.Add(entity.NewMonster(req.Kind))
	}
}

// Enqueue schedules a job on the world's job goroutine.
func (w *World) Enqueue(job func()) {
	w.jobs <- job
}

func (w *World) runJobs() {
	for job := range w.jobs {
		job()
	}
}

func (w *World) saveSnapshot() {
	w.mu.Lock()
	defer w.mu.Unlock()
	w.score.snapshots++
}
