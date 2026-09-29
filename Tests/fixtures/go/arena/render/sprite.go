package render

// Sprite is a static image drawn with a value receiver.
type Sprite struct {
	Name string
	X, Y float64
}

// Draw renders the sprite.
func (s Sprite) Draw(c *Canvas) {
	c.Put(s.Name, s.X, s.Y)
}
