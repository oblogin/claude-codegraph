package entity

// Crate is a static prop: it has an id but no per-tick logic.
type Crate struct {
	Base
	Loot string
}

// NewCrate creates a crate with loot inside.
func NewCrate(loot string) *Crate {
	return &Crate{Base: newBase(), Loot: loot}
}
