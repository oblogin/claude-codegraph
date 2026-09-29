// Package ui contains overlay widgets.
package ui

import r "example.com/arena/render"

// Tooltip is a floating hint. The game only ever draws it directly.
type Tooltip struct {
	Text string
	X, Y float64
}

// Draw renders the tooltip.
func (t *Tooltip) Draw(c *r.Canvas) {
	c.Put(t.Text, t.X, t.Y)
}

// Move shifts the tooltip.
func (t *Tooltip) Move(dx, dy float64) {
	t.X += dx
	t.Y += dy
}
