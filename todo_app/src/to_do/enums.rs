use std::fmt;

pub enum TaskStatus {
    DONE,
    PENDING,
}

impl TaskStatus {
    pub fn from_string(value: String) -> Self {
         match value.to_uppercase().as_str() {
            "DONE" => TaskStatus::DONE,
            "PENDING" => TaskStatus::PENDING,
            _ => panic!("input {value} not supported"),
         }
    }

    pub fn stringify(&self) -> String {
        match &self {
            &Self::DONE => String::from("DONE"),
            &Self::PENDING => String::from("PENDING"),
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            &Self::DONE => {
                write!(f, "DONE")
            }
            &Self::PENDING => {
                write!(f, "PENDING")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn println_done() {
        println!("{}", TaskStatus::DONE);
    }

    #[test]
    fn println_pending() {
        println!("{}", TaskStatus::PENDING);
    }

    #[test]
    fn string_done() {
        let expected = String::from("DONE");
        let actual = TaskStatus::DONE.to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn string_pending() {
        let expected = String::from("PENDING");
        let actual = TaskStatus::PENDING.to_string();
        assert_eq!(actual, expected);
    }
}
