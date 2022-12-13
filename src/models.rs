use deisel::prelude::*;

#[derive(Queryable)]
pub struct Puck {
    pub id: i32,
    pub name: String,
    pub path: String,
}