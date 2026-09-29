package geom

import "testing"

func TestAdd(t *testing.T) {
	got := Vec{X: 1, Y: 2}.Add(Vec{X: 3, Y: 4})
	if got != (Vec{X: 4, Y: 6}) {
		t.Fatalf("got %v", got)
	}
}

func TestScale(t *testing.T) {
	v := Vec{X: 1, Y: 2}
	v.Scale(2)
	if v.X != 2 || v.Y != 4 {
		t.Fatalf("got %v", v)
	}
}
