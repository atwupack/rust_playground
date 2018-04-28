use reader::*;

#[test]
fn test_ask() {
    let env = Env { value: 1};
    let e = ask();
    let r = (e.run_reader)(&env);
    println!("{:?}", r)
}

#[derive(Clone, Debug)]
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
    let r = (me.run_reader)(&env);
    println!("{}", r)
}

fn some_reader_func<'a>(input: i32) -> Reader<'a, Env, i32> {
    let e = ask();
    e.map(move |r: Env| r.value + input)
}

#[test]
fn test_bind() {

    let env = Env { value: 1};
    let e = ask();
    let me = e.map(|r: Env| {
        r.value
    });
    let be = me.bind(some_reader_func);

    let r = (be.run_reader)(&env);
    println!("test_bind: {}", r)
}
