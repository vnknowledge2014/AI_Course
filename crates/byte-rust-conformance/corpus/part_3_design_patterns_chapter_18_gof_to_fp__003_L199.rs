// filename: src/main.rs

type Listener<T> = Box<dyn Fn(&T)>;

struct EventEmitter<T> {
    listeners: Vec<Listener<T>>,
}

impl<T> EventEmitter<T> {
    fn new() -> Self { EventEmitter { listeners: vec![] } }

    fn on(mut self, handler: impl Fn(&T) + 'static) -> Self {
        self.listeners.push(Box::new(handler));
        self
    }

    fn emit(&self, event: &T) {
        for listener in &self.listeners {
            listener(event);
        }
    }
}

#[derive(Debug)]
enum AppEvent {
    UserLogin(String),
    PageView(String),
    Purchase { item: String, amount: u32 },
}

fn main() {
    let emitter = EventEmitter::new()
        .on(|e: &AppEvent| println!("  [Logger] {:?}", e))
        .on(|e: &AppEvent| {
            if let AppEvent::Purchase { amount, .. } = e {
                if *amount > 100_000 {
                    println!("  [Alert] Large purchase: {}đ!", amount);
                }
            }
        })
        .on(|e: &AppEvent| {
            if let AppEvent::UserLogin(name) = e {
                println!("  [Welcome] Hello, {}!", name);
            }
        });

    println!("Event 1:");
    emitter.emit(&AppEvent::UserLogin("Minh".into()));

    println!("\nEvent 2:");
    emitter.emit(&AppEvent::Purchase { item: "Laptop".into(), amount: 25_000_000 });

    println!("\nEvent 3:");
    emitter.emit(&AppEvent::PageView("/products".into()));
}
