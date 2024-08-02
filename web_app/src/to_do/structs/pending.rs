use crate::to_do::enums::TaskStatus;
use crate::to_do::structs::base::Base;

pub struct Pending {
    pub super_struct: Base,
}
impl Pending {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: String::from(input_title),
            status: TaskStatus::PENDING,
        };
        return Pending { super_struct: base };
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn new() {
        let new_base_struct = Pending::new("test title");
        assert_eq!(
            String::from("test title"),
            new_base_struct.super_struct.title
        );
        assert_eq!(TaskStatus::PENDING, new_base_struct.super_struct.status);
    }
}
