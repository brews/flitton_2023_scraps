use serde_json::value::Value;
use serde_json::Map;

use crate::state::write_to_file;

pub trait Delete {
    fn delete(&self, title: &str, state: &mut Map<String, Value>) {
        state.remove(title);
        write_to_file("./state.json", state);
        println!("\n\n{title} is being deleted\n\n");
    }
}
