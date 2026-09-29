package ui

import r "example.com/arena/render"

// Button is a clickable label.
type Button struct {
	Label string
}

// Draw renders the button, optionally highlighted.
func (b Button) Draw(c *r.Canvas, highlighted bool) {
	label := b.Label
	if highlighted {
		label = "*" + label
	}
	c.Put(label, 0, 0)
}

// ghost is an invisible widget kept for debugging.
type ghost struct{}

func (g ghost) draw(c *r.Canvas) {
	c.Put("ghost", 0, 0)
}
