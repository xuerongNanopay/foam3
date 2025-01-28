pub(crate) trait KeyOrd {
    fn compare(&self, search_key: (*const u8, usize), tree_key: (*const u8, usize)) -> i32;
}
