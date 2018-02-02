use monad::Monad;

fn positive_value(input: &i32) -> Option<i32> {
    if *input > 0 {
        Some(*input)
    } else {
        None
    }
}

// #[test]
// fn test_option() {
//     let op: Option<i32> = Some(0);
//     let result = op.bind(positive_value);
//     assert_eq!(result, None);

//     let op: Option<i32> = Some(1);
//     let result = op.bind(positive_value);
//     assert_eq!(result, Some(1));

//     let result : Option<i32> = Option::mpure(1);
// }
