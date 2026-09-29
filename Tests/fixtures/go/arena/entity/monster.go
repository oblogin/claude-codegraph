package entity

import "example.com/arena/geom"

// Monster is an enemy driven by the world. It embeds a pointer to Base.
type Monster struct {
	*Base
	Kind  string
	speed float64
}

// NewMonster creates a monster of the given kind.
func NewMonster(kind string) *Monster {
	b := newBase()
	return &Monster{Base: &b, Kind: kind, speed: 2}
}

// Update walks the monster to the left.
func (m *Monster) Update(dt float64) {
	m.Move(geom.Vec{X: -m.speed * dt})
}

// Damage reports whether a hit of size n is lethal.
func (m Monster) Damage(n int) bool {
	return n >= 10
}
