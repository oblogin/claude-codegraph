// Package events is a tiny publish/subscribe bus.
package events

// Topic names.
const (
	TopicKilled  = "killed"
	TopicSpawned = "spawned"
)

// Event is delivered to handlers.
type Event struct {
	Topic   string
	Payload any
}

// Handler receives events.
type Handler func(e Event)

// Bus dispatches events to subscribed handlers.
type Bus struct {
	handlers map[string][]Handler
}

// NewBus creates an empty bus.
func NewBus() *Bus {
	return &Bus{handlers: map[string][]Handler{}}
}

// Subscribe registers h for topic.
func (b *Bus) Subscribe(topic string, h Handler) {
	b.handlers[topic] = append(b.handlers[topic], h)
}

// Publish calls every handler subscribed to topic.
func (b *Bus) Publish(topic string, payload any) {
	for _, h := range b.handlers[topic] {
		h(Event{Topic: topic, Payload: payload})
	}
}
