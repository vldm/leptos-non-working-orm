//!
//! This module emulate ORM work, implementing query for some type.
//!

/// Do some raw query bounded to type, and return array of all elements
/// in real world example can query `SELECT * from X` and map result to a type.
/// But in mocked version just returns vector of mocked data.
pub trait SelectFromDb {
    fn mock_select_all() -> Vec<(u32, Self)>
    where
        Self: Sized;
}
