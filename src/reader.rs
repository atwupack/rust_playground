pub struct Reader<'a, E, R> {
    pub run_reader: Box<Fn(E) -> R + 'a>,
}

pub fn ask<'a, E>() -> Reader<'a, E, E> {
    Reader {
        run_reader: Box::new(|e| e),
    }
}

impl<'a, E: 'a, R: 'a> Reader<'a, E, R> {
    pub fn map<R2, F>(self, f: F) -> Reader<'a, E, R2>
    where
        F: Fn(R) -> R2 + 'a,
    {
        Reader {
            run_reader: Box::new(move |e| {
                let r = (self.run_reader)(e);
                f(r)
            }),
        }
    }

    pub fn bind<R2, F>(self, f: F) -> Reader<'a, E, R2>
    where
        F: Fn(R) -> Reader<'a, E, R2> + 'a,
        E: Clone,
    {
        Reader {
            run_reader: Box::new(move |e| {
                let r = (self.run_reader)(e.clone());
                let inner_reader = f(r);
                (inner_reader.run_reader)(e)
            }),
        }
    }
}
