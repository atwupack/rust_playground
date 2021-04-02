pub struct Reader<'a, E: 'a, R> {
    pub run_reader: Box<dyn Fn(&E) -> R + 'a>,
}

pub fn ask<'a, E: Clone>() -> Reader<'a, E, E> {
    Reader {
        run_reader: Box::new(|e| e.clone()),
    }
}

// pub fn rpure<'a, E, T: 'a>(input: T) -> Reader<'a, E, T> {
//     Reader {
//         run_reader: Box::new(|e| input),
//     }
// }


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
    {
        Reader {
            run_reader: Box::new(move |e| {
                let r = (self.run_reader)(e);
                let inner_reader = f(r);
                (inner_reader.run_reader)(e)
            }),
        }
    }
}
