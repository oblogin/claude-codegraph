// Package entity defines game objects.
package entity

import "example.com/arena/geom"

// Identified is anything with a stable id.
type Identified interface {
	ID() int
}

// Entity is updated by the world every tick.
type Entity interface {
	Identified
	Update(dt float64)
}

// Base carries fields shared by all game objects.
type Base struct {
	id  int
	Pos geom.Vec
}

// ID returns the object id.
func (b *Base) ID() int {
	return b.id
}

// Move shifts the object by d.
func (b *Base) Move(d geom.Vec) {
	b.Pos = b.Pos.Add(d)
}

var nextID = 0

func newBase() Base {
	nextID++
	return Base{id: nextID}
}
