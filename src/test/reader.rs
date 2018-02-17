use reader::*;

#[test]
fn test_ask() {
    let e = ask();
    let r = (e.run_reader)("env_string".to_owned());
    println!("{}", r)
}

struct Env {
    value: i32,
}

#[test]
fn test_map() {

    let env = Env { value: 1};

    let e = ask();
    let me = e.map(|r: Env| {
        r.value
    });
    let r = (me.run_reader)(env);
    println!("{}", r)
}
