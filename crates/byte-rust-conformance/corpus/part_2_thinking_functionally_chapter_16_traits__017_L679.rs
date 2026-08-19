trait Plugin {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn execute(&self, input: &str) -> Result<String, String>;
}

struct UpperPlugin;
impl Plugin for UpperPlugin {
    fn name(&self) -> &str { "uppercase" }
    fn version(&self) -> &str { "1.0" }
    fn execute(&self, input: &str) -> Result<String, String> {
        Ok(input.to_uppercase())
    }
}

struct ReversePlugin;
impl Plugin for ReversePlugin {
    fn name(&self) -> &str { "reverse" }
    fn version(&self) -> &str { "1.0" }
    fn execute(&self, input: &str) -> Result<String, String> {
        Ok(input.chars().rev().collect())
    }
}

struct CountPlugin;
impl Plugin for CountPlugin {
    fn name(&self) -> &str { "word-count" }
    fn version(&self) -> &str { "1.0" }
    fn execute(&self, input: &str) -> Result<String, String> {
        Ok(format!("{} words", input.split_whitespace().count()))
    }
}

struct PluginRunner {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginRunner {
    fn new() -> Self { PluginRunner { plugins: vec![] } }

    fn register(mut self, plugin: Box<dyn Plugin>) -> Self {
        self.plugins.push(plugin);
        self
    }

    fn run_all(&self, input: &str) {
        println!("Input: '{}'\n", input);
        for plugin in &self.plugins {
            match plugin.execute(input) {
                Ok(result) => println!("  [{}@{}] → {}", plugin.name(), plugin.version(), result),
                Err(e) => println!("  [{}] ❌ {}", plugin.name(), e),
            }
        }
    }
}

fn main() {
    let runner = PluginRunner::new()
        .register(Box::new(UpperPlugin))
        .register(Box::new(ReversePlugin))
        .register(Box::new(CountPlugin));

    runner.run_all("Hello Rust World");
}
