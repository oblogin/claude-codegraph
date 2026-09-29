// Package geom holds 2D math helpers.
package geom

import "math"

// Vec is a 2D vector.
type Vec struct {
	X, Y float64
}

// Add returns the sum of two vectors (value receiver).
func (v Vec) Add(o Vec) Vec {
	return Vec{X: v.X + o.X, Y: v.Y + o.Y}
}

// Scale multiplies the vector in place (pointer receiver).
func (v *Vec) Scale(k float64) {
	v.X *= k
	v.Y *= k
}

// Len returns the vector length.
func (v Vec) Len() float64 {
	return math.Hypot(v.X, v.Y)
}
