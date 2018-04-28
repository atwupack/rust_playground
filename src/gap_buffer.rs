use std::ptr::copy;
use std::convert::{From, Into};
use std::vec::Vec;

#[derive(Debug)]
pub struct GapBuffer<T> {
    data: Vec<T>,
    gap_start: usize,
    gap_length: usize,
}

impl<T: Copy> Into<Vec<T>> for GapBuffer<T> {
    fn into(mut self) -> Vec<T> {
        let len = self.length();
        self.move_gap(len);
        unsafe {
            self.data.set_len(len);
        }
        self.data
    }
}

impl<T: Copy> From<Vec<T>> for GapBuffer<T> {
    /// Create a `GapBuffer` from a given vector.
    fn from(mut data: Vec<T>) -> Self {
        let gap_start = data.len();
        let gap_length = data.capacity() - gap_start;
        let capacity = data.capacity();

        unsafe {
            data.set_len(capacity);
        }
        GapBuffer {
            data: data,
            gap_start: gap_start,
            gap_length: gap_length,
        }
    }
}

impl<T: Copy> GapBuffer<T> {
    /// Create a new `GapBuffer` with the given capacity.
    pub fn empty(capacity: usize) -> Self {
        let mut v = Vec::with_capacity(capacity);
        unsafe {
            v.set_len(capacity);
        }
        GapBuffer {
            data: v,
            gap_start:0,
            gap_length: 0,
        }
    }

    pub fn insert(&mut self, data: &[T], index: usize) {
        if data.len() == 0 {
            return;
        }
        //self.check_index(index);
        if self.gap_length < data.len() {
            let add = data.len() - self.gap_length;
            self.reserve(add);
        }
        if self.gap_start != index {
            self.move_gap(index);
        }
        unsafe {
            let dest_ptr = self.data.as_mut_ptr();
            copy(data.as_ptr(), dest_ptr.offset(index as isize), data.len());
        }
        self.gap_length -= data.len();
        self.gap_start += data.len();
    }


    /// Create a vector containing the `GapBufer`'s data.
    pub fn to_vec(&mut self) -> Vec<T> {
        let vec_size = self.data.len() - self.gap_length;
        let mut result = Vec::with_capacity(vec_size);
        let second_start = self.gap_start + self.gap_length;
        unsafe {
            result.set_len(vec_size);
            copy(self.data.as_mut_ptr(),result.as_mut_ptr(), self.gap_start);
            copy(self.data.as_mut_ptr().offset(second_start as isize), result.as_mut_ptr().offset(self.gap_start as isize), self.data.len() - second_start);
        }
        result
    }

    /// Checks if an index is inside the data range.
    fn check_index(&self, index: usize) {
        if index >= self.length() {
            panic!("Index {} is outside buffer size of {}.", index, self.length());
        }
    }

    /// Remove a section of data from the `GapBuffer`.
    pub fn remove(&mut self, start: usize, count: usize) {
        if count == 0 {
            return;
        }
        self.check_index(start);
        let end = start + count - 1;
        self.check_index(end);
        if start > self.gap_start {
            // move the gap to have to be removed data after it
            self.move_gap(start);
        }
        else if end < self.gap_start - 1 {
            // move the gap to have to be removed data before it
            self.move_gap(end - 1);
        }
        self.gap_start = start;
        self.gap_length += count;
    }

    /// Increase capacity of `GapBuffer` to hold addional data.
    pub fn reserve(&mut self, additional: usize) {
        let old_capacity = self.data.capacity();
        self.data.reserve_exact(additional);
        let new_capacity = self.data.capacity();
        let actual_add = new_capacity - old_capacity;
        let old_second_start = self.gap_start + self.gap_length;
        unsafe {
            self.data.set_len(new_capacity);
            self.copy_data(old_second_start, old_second_start + actual_add, old_capacity - old_second_start);
        }

        self.gap_length += actual_add;
    }

    pub fn move_gap(&mut self, new_gap_start: usize) {
        if new_gap_start==self.gap_start || self.gap_length==0 {
            self.gap_start = new_gap_start;
            return;
        }
        let max_gap_start = self.data.len() - self.gap_length;
        if new_gap_start > max_gap_start {
            panic!("Cannot move gap beyond end of buffer.");
        }
        let second_start = self.gap_start + self.gap_length;
        let old_gap_start = self.gap_start;
        if new_gap_start < self.gap_start {
            let copy_count = self.gap_start - new_gap_start;
            unsafe {
                self.copy_data(new_gap_start, second_start-copy_count, copy_count)
            }
        }
        else {
            let copy_count = new_gap_start - self.gap_start;
            unsafe {
                self.copy_data(second_start, old_gap_start ,copy_count)
            }
        }
        self.gap_start = new_gap_start;
    }

    /// Copy data inside buffer.
    unsafe fn copy_data(&mut self, src: usize, dest: usize, count: usize) {
        if count == 0 {
            return;
        }
        let ptr = self.data.as_mut_ptr();
        copy(ptr.offset(src as isize), ptr.offset(dest as isize), count);
    }

    /// The capacity of the `GapBuffer`.
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// The length of the data in the `GapBuffer`.
    pub fn length(&self) -> usize {
        self.capacity() - self.gap_length
    }
}
