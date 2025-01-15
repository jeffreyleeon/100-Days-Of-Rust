use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

trait MessageProvider {
    fn get_message(&self) -> String;
}

struct HelloWorldProvider;

impl MessageProvider for HelloWorldProvider {
    fn get_message(&self) -> String {
        "Hello world".to_string()
    }
}

struct OutputManager {
    message_provider: Arc<dyn MessageProvider + Send + Sync>,
}

impl OutputManager {
    fn new(provider: Arc<dyn MessageProvider + Send + Sync>) -> Self {
        OutputManager { message_provider: provider }
    }

    fn display_message(&self) {
        let message = self.message_provider.get_message();
        println!("{}", message);
    }
}

fn main() {
    let provider = Arc::new(HelloWorldProvider);
    let output_manager = Arc::new(Mutex::new(OutputManager::new(provider)));
    
    let output_thread = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        let manager = output_manager.lock().unwrap();
        manager.display_message();
    });

    output_thread.join().unwrap();
}
