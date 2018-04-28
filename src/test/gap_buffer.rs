use gap_buffer::GapBuffer;
use std::convert::From;

fn create_test_buffer() -> GapBuffer<i32> {
    let v = vec![0,1,2,3,4,5,6,7,8,9];
    let mut b = GapBuffer::from(v);
    assert_eq!(b.to_vec(), vec![0,1,2,3,4,5,6,7,8,9]);
    b.reserve(5);
    assert_eq!(b.to_vec(), vec![0,1,2,3,4,5,6,7,8,9]);
    b.move_gap(5);
    assert_eq!(b.to_vec(), vec![0,1,2,3,4,5,6,7,8,9]);
    b
}

#[test]
fn test_remove_start() {
    let mut b = create_test_buffer();
    b.remove(1,2);
    assert_eq!(b.length(), 8);
    assert_eq!(b.to_vec(), vec![0,3,4,5,6,7,8,9]);
}

#[test]
fn test_remove_end() {
    let mut b = create_test_buffer();
    let len = b.length();
    b.remove(len-1 ,1);
    assert_eq!(b.length(), 9);
    assert_eq!(b.to_vec(), vec![0,1,2,3,4,5,6,7,8]);
}

#[test]
fn test_remove_over_gap() {
    let mut b = create_test_buffer();
    b.remove(3,5);
    assert_eq!(b.length(), 5);
    assert_eq!(b.to_vec(), vec![0,1,2,8,9]);
}

#[test]
fn test_remove_all() {
    let mut b = create_test_buffer();
    b.remove(0,10);
    assert_eq!(b.length(), 0);
    assert_eq!(b.to_vec(), Vec::new());
}

#[test]
#[should_panic]
fn test_remove_too_much() {
    let mut b = create_test_buffer();
    b.remove(4,10);
}

#[test]
#[should_panic]
fn test_remove_outside() {
    let mut b = create_test_buffer();
    b.remove(10,5);
}


#[test]
fn test_insert_start() {
    let mut b = create_test_buffer();
    let new_data = &[11,12,13,14,15];
    b.insert(new_data, 0);
    assert_eq!(b.length(), 15);
    assert_eq!(b.to_vec(), vec![11,12,13,14,15,0,1,2,3,4,5,6,7,8,9]);
}

#[test]
fn test_insert_end() {
    let mut b = create_test_buffer();
    let new_data = &[11,12,13,14,15];
    let len = b.length();
    b.insert(new_data, len);
    assert_eq!(b.length(), 15);
    assert_eq!(b.to_vec(), vec![0,1,2,3,4,5,6,7,8,9,11,12,13,14,15]);
}

#[test]
fn test_insert_gap() {
    let mut b = create_test_buffer();
    let new_data = &[11,12,13,14,15,16,17,18,19];
    b.insert(new_data, 3);
    assert_eq!(b.length(), 19);
    assert_eq!(b.to_vec(), vec![0,1,2,11,12,13,14,15,16,17,18,19,3,4,5,6,7,8,9]);
}

#[test]
#[should_panic]
fn test_insert_after_end() {
    let mut b = create_test_buffer();
    let new_data = &[11,12,13,14,15,16,17,18,19];
    b.insert(new_data, 11);
}