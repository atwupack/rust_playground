use impl_trait::*;

#[test]
fn test_trait_bounds() {
    trait_bound_1("input");
    trait_bound_2("input");
    // trait_bound_3("input");
    println!("{}", trait_bound_4("input"));
    // println!("{}", trait_bound_5());
}