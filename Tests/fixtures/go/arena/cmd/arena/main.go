// Command arena runs a tiny headless game loop.
package main

import (
	"fmt"

	"example.com/arena/entity"
	"example.com/arena/events"
	"example.com/arena/game"
	"example.com/arena/geom"
	"example.com/arena/render"
	"example.com/arena/ui"
)

func main() {
	w := game.NewWorld()
	player := entity.NewPlayer("hero")
	player.Vel = geom.Vec{X: 1}
	w.Add(player)

	crate := entity.NewCrate("gold")
	fmt.Println("crate", crate.ID())

	requests := make(chan game.SpawnRequest)
	w.StartSpawner(requests)
	go func() {
		for _, kind := range []string{"slime", "bat"} {
			requests <- game.SpawnRequest{Kind: kind}
		}
		close(requests)
	}()

	w.Bus.Subscribe(events.TopicKilled, ui.OnMonsterKilled)
	w.Bus.Subscribe(events.TopicSpawned, func(e events.Event) {
		fmt.Println("spawned", e.Payload)
	})

	hud := &ui.HUD{}
	tip := &ui.Tooltip{Text: "press space"}
	for i := 0; i < 3; i++ {
		w.Tick(0.016)
		hud.Update(0.016)
		tip.Move(1, 0)
		tip.Draw(w.Canvas)
	}
	ui.Button{Label: "ok"}.Draw(w.Canvas, true)
	w.Canvas.DrawAll([]render.Drawer{render.Sprite{Name: "tree", X: 5}})
	player.Damage(5)
	w.Kill(entity.NewMonster("boss"))
	fmt.Println(hud.Line(), len(w.Canvas.Commands))
}
