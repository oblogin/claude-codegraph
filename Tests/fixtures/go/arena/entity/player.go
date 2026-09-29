package entity

import (
	"example.com/arena/geom"
	"example.com/arena/render"
)

// Player is controlled by the user.
type Player struct {
	Base
	Name string
	Vel  geom.Vec
	hp   int
}

// NewPlayer creates a player with full health.
func NewPlayer(name string) *Player {
	return &Player{Base: newBase(), Name: name, hp: 100}
}

// Update moves the player and applies friction.
func (p *Player) Update(dt float64) {
	p.Move(geom.Vec{X: p.Vel.X * dt, Y: p.Vel.Y * dt})
	p.Vel.Scale(0.9)
}

// Damage reduces health.
func (p *Player) Damage(n int) {
	p.hp -= n
}

// Draw renders the player.
func (p *Player) Draw(c *render.Canvas) {
	c.Put(p.Name, p.Pos.X, p.Pos.Y)
}
