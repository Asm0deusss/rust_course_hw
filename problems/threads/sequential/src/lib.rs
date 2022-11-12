#![forbid(unsafe_code)]
use std::{
    sync::{Arc, Mutex},
    thread::{self, Thread},
    time::Duration,
};

pub fn sequential_run<Printer, Iter>(
    printer: Printer,
    iterator: Iter,
    odd_thread_ms: u64,
    even_thread_ms: u64,
    max_iterations: usize,
) where
    Iter: Iterator + Sync + Send + 'static,
    Printer: Fn(&Thread, Iter::Item) + Sync + Send + 'static,
{
    let mutex_iterator_index_1 = Arc::new(Mutex::new(0_usize));
    let mutex_iterator_index_2 = mutex_iterator_index_1.clone();

    let max_iter_1 = Arc::new(max_iterations);
    let max_iter_2 = max_iter_1.clone();

    let printer_1 = Arc::new(printer);
    let printer_2 = printer_1.clone();

    let mutex_it_1 = Arc::new(Mutex::new(iterator));
    let mutex_it_2 = mutex_it_1.clone();

    let dead_1_1 = Arc::new(Mutex::new(false));
    let dead_2_1 = dead_1_1.clone();

    let dead_1_2 = Arc::new(Mutex::new(false));
    let dead_2_2 = dead_1_2.clone();

    let odd_thread = std::thread::Builder::new()
        .name("odd thread".to_string())
        .spawn(move || loop {
            thread::sleep(Duration::from_millis(odd_thread_ms));

            let mut cur_iter_index = mutex_iterator_index_1.lock().unwrap();

            let dead = dead_1_2.lock().unwrap();

            if *dead {
                println!("DEAD first");
                break;
            }

            if *cur_iter_index >= *max_iter_1 {
                break;
            }

            if *cur_iter_index % 2 == 0 {
                continue;
            }

            let mut cur_iterator = mutex_it_1.lock().unwrap();
            let next = cur_iterator.next();

            if let Some(..) = next {
                printer_1(&thread::current(), next.unwrap());
                *cur_iter_index += 1;
            } else {
                let mut death = dead_1_1.lock().unwrap();
                *death = true;
                break;
            }
        })
        .unwrap();

    let even_thread = std::thread::Builder::new()
        .name("even thread".to_string())
        .spawn(move || loop {
            thread::sleep(Duration::from_millis(even_thread_ms));

            let mut cur_iter_index = mutex_iterator_index_2.lock().unwrap();

            let dead = dead_2_1.lock().unwrap();

            if *dead {
                println!("DEAD secons");
                break;
            }

            if *cur_iter_index >= *max_iter_2 {
                break;
            }

            if *cur_iter_index % 2 == 1 {
                continue;
            }

            let mut cur_iterator = mutex_it_2.lock().unwrap();
            let next = cur_iterator.next();

            if let Some(..) = next {
                printer_2(&thread::current(), next.unwrap());
                *cur_iter_index += 1;
            } else {
                let mut death = dead_2_2.lock().unwrap();
                *death = true;
                break;
            }
        })
        .unwrap();

    odd_thread.join().unwrap();
    even_thread.join().unwrap();
}
