use parser::*;

#[test]
fn test_any_token_parser() {
    let atp = any_token();
    let atp = map(atp, |x| x+1);
    let input = vec![1,2,3];
    let mut iter = input.into_iter();
    let r = atp.run(&mut iter);
    println!("{:?}", r);
    let r = atp.run(&mut iter);
    println!("{:?}", r);
    let r = atp.run(&mut iter);
    println!("{:?}", r);
    let r = atp.run(&mut iter);
    println!("{:?}", r);
}