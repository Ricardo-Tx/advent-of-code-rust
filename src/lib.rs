pub mod template;

// Use this file to add helper functions and additional modules.
pub trait One {
    fn one() -> Self;
}

impl One for i32 {
    fn one() -> Self {
        1
    }
}

impl One for usize {
    fn one() -> Self {
        1
    }
}

pub fn point_to_str<T: Copy + std::ops::Add<Output = T> + std::fmt::Display + One>(point: &[T;2]) -> String {
    let one : T = T::one();
    format!("Ln {}, Col {}", point[1]+one, point[0]+one)
}
