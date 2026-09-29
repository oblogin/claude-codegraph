// Package render draws things onto a text canvas.
package render

import "fmt"

// Drawer is anything that can draw itself.
type Drawer interface {
	Draw(c *Canvas)
}

// Canvas collects draw commands.
type Canvas struct {
	Commands []string
}

// Put records a labelled point.
func (c *Canvas) Put(label string, x, y float64) {
	c.Commands = append(c.Commands, fmt.Sprintf("%s@%.0f,%.0f", label, x, y))
}

// DrawAll draws every item through the Drawer interface.
func (c *Canvas) DrawAll(items []Drawer) {
	for _, it := range items {
		it.Draw(c)
	}
}
