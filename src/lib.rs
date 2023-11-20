#![allow(dead_code)]
#![allow(unused_imports)]

use std::sync::{Arc, Condvar, Mutex};

pub struct Sender<T> {
    inner: Arc<Mutex<Inner<T>>>,
}

pub struct Receiver<T> {
    inner: Arc<Mutex<Inner<T>>>,
}

struct Inner<T> {
    queue: Mutex<Vec<T>>,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: Mutex::default(),
    };

    let inner = Arc::new(Mutex::new(inner));
    (
        Sender {
            inner: inner.clone(),
        },
        Receiver {
            inner: inner.clone(),
        }
    )
}
