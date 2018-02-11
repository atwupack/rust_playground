use std::collections::VecDeque;

pub struct RollbackIterator<I>
    where
        I: Iterator,
{
    iter: I,
    cache: VecDeque<I::Item>,
    running: bool,
}


impl<I> Iterator for RollbackIterator<I>
    where
        I: Iterator,
        I::Item : Copy,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.running {
            // transaction is running
            // just get the next item from the underlying iterator and save it
            let item = self.iter.next();
            match item {
                Some(i) => {
                    self.cache.push_back(i);
                    Some(i)
                },
                None => None,
            }
        } else {
            // no transaction running
            // if cache it empty, just deliver the next item
            // else get the last entry from the cache
            if self.cache.is_empty() {
                self.iter.next()
            } else {
                self.cache.pop_front()
            }
        }
    }
}

impl<I> RollbackIterator<I>
    where
        I: Iterator,
{
    pub fn new(input: I) -> RollbackIterator<I> {
        RollbackIterator {
            iter: input,
            running: false,
            cache: VecDeque::new(),
        }
    }

    pub fn start_trans(&mut self) {
        self.cache.clear();
        self.running = true;
    }

    pub fn rollback(&mut self) {
        self.running = false;
    }

    pub fn commit(&mut self) {
        self.running = false;
        self.cache.clear();
    }
}