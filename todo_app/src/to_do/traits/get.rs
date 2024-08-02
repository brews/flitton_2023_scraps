use serde_json::value::Value;
use serde_json::Map;

pub trait Get {
    fn get(&self, title: &str, state: &Map<String, Value>) {
        let item: Option<&Value> = state.get(title);

        match item {
            Some(result) => {
                println!("\n\nItem: {title}");
                println!("Status: {result}\n\n");
            }
            None => println!("item: {title} was not found"),
        }
    }
}
