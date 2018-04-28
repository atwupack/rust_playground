use std::fmt::Display;

pub fn trait_bound_1<T: Display>(input: T) {
    println!("{}", input)
}

pub fn trait_bound_2<T>(input: T)
    where
        T: Display,
{
    println!("{}", input)
}

// pub fn trait_bound_3(input: impl Display) {
//     println!("{}", input)
// }

pub fn trait_bound_4<T>(input: T) -> T {
    input
}

// pub fn trait_bound_5() -> impl Display {
//     String::from("input")
// }