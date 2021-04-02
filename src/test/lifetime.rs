

struct Context<'s>(&'s str);

struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

#[test]
fn test_parser() {
    let c = Context("Hello world!");
    parse_context(c).expect_err("OK");
}

trait Foo { }

struct Bar<'a> {
    _x: &'a i32,
}

impl<'a> Foo for Bar<'a> { }

#[test]
fn test_trait_object() {
    let num = 5;
    let _obj = Box::new(Bar { _x: &num }) as Box<dyn Foo>;
}

