use crate::to_do::enums::TaskStatus;
use crate::to_do::structs::base::Base;
use crate::to_do::traits::create::Create;
use crate::to_do::traits::edit::Edit;
use crate::to_do::traits::get::Get;

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

impl Get for Pending {}
impl Create for Pending {}
impl Edit for Pending {}
