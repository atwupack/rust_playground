use functor::Functor;

#[test]
fn test_option() {
    let op = Some(0);
    let result = op.fmap(|x| x + 1);
    assert_eq!(result, Some(1));
}

#[test]
fn test_result() {
    let ok_result: Result<i32, &str> = Ok(0);
    let result = ok_result.fmap(|x| x + 1);
    assert_eq!(result, Ok(1));

    let err_result: Result<i32, &str> = Err("Error");
    let result = err_result.fmap(|x| x + 1);
    assert_eq!(result, Err("Error"));
}

#[test]
fn test_vec() {
    let v = vec![1,2,3,4,5];
    let result = v.fmap(|x| x + 1);
    assert_eq!(result, vec![2,3,4,5,6]);
}

