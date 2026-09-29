export type Handler<T> = (payload: T) => void;

export class EventEmitter<Events extends Record<string, unknown>> {
  private readonly handlers = new Map<keyof Events, Set<Handler<any>>>();

  on<K extends keyof Events>(name: K, handler: Handler<Events[K]>): void {
    let set = this.handlers.get(name);
    if (!set) {
      set = new Set<Handler<any>>();
      this.handlers.set(name, set);
    }
    set.add(handler);
  }

  off<K extends keyof Events>(name: K, handler: Handler<Events[K]>): void {
    this.handlers.get(name)?.delete(handler);
  }

  emit<K extends keyof Events>(name: K, payload: Events[K]): void {
    const set = this.handlers.get(name);
    if (!set) {
      return;
    }
    for (const handler of set) {
      handler(payload);
    }
  }
}
