use iterator::RollbackIterator;

#[test]
fn test_rollback() {
    let input = vec![1,2,3];
    let iter = input.into_iter();
    let mut rb_iter = RollbackIterator::new(iter);
    assert_eq!(rb_iter.next(), Some(1));
    rb_iter.start_trans();
    assert_eq!(rb_iter.next(), Some(2));
    assert_eq!(rb_iter.next(), Some(3));
    rb_iter.rollback();
    assert_eq!(rb_iter.next(), Some(2));
    assert_eq!(rb_iter.next(), Some(3));
    assert_eq!(rb_iter.next(), None);
}

#[test]
fn test_commit() {
    let input = vec![1,2,3,4];
    let iter = input.into_iter();
    let mut rb_iter = RollbackIterator::new(iter);
    assert_eq!(rb_iter.next(), Some(1));
    rb_iter.start_trans();
    assert_eq!(rb_iter.next(), Some(2));
    assert_eq!(rb_iter.next(), Some(3));
    rb_iter.commit();
    assert_eq!(rb_iter.next(), Some(4));
    assert_eq!(rb_iter.next(), None);
}
