use parser::*;
use parser::prim::*;

#[test]
fn test_any_token_parser() {
    let atp = any_token();
    let atp = atp.map(|x| x+1);
    let input = vec![1,2,3];
    let mut iter = input.into_iter();
    let r = atp.run(&mut iter);
    println!("{:?}", r);
    let atp = any_token();
    let atp = atp.map(|x| x+1);
    let r = atp.run(&mut iter);
    println!("{:?}", r);
    let atp = any_token();
    let atp = atp.map(|x| x+1);
    let r = atp.run(&mut iter);
    println!("{:?}", r);
    let atp = any_token();
    let atp = atp.map(|x| x+1);
    let r = atp.run(&mut iter);
    println!("{:?}", r);
}

#[test]
fn test_seq() {
    let input = vec![1,2,3];
    let mut iter = input.into_iter();

    let atp = any_token().seq(any_token()).seq(any_token());

    let r = atp.run(&mut iter);
    println!("{:?}", r);
}