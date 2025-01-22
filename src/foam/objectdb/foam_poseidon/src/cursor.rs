#![allow(unused)]

trait Cursor {
    fn get_key(&self);
    fn get_value(&self);
    fn set_key(&self);
    fn set_value(&self);
    fn next(&self);
    fn prev(&self);
    fn insert(&self);
    fn update(&self);
    fn remove(&self);
}