#[derive(Debug)]
pub enum ProductCountAvailable {
  Available {
    is_available: bool,
  },
  All,
}
