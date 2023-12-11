pub trait Error<T: Default, E> {
    fn error(self) -> T;
}

impl<T: Default, E: std::fmt::Display> Error<T, E> for Result<T, E> {
    fn error(self) -> T {
        return match self {
            Ok(result) => result,
            Err(error) => {
                println!("No Result {}", error);
                T::default()
            }
        };
    }
}

impl<T: Default, E: std::fmt::Display> Error<T, E> for Option<T> {
    fn error(self) -> T {
        return match self {
            Some(result) => result,
            None => {
                println!("No Option");
                T::default()
            }
        };
    }
}
