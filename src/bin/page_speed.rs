use memmap::MmapOptions;
use std::fs::File;
use std::thread;


fn main() {
    let handle = thread::spawn(|| {
        for i in 0..250 {
            read_page(i)
        }
    });

    // assert_eq!(b"# memmap", &mmap[0..8]);
    for i in 250..500 {
        read_page(i)
    }
    handle.join().unwrap();
}

fn read_page(i: usize) {
    let file = File::open("/home/daoud/data/tinysearch/abstract-titles-sorted.txt.gz")
        .unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    let page_num = i * 10;
    let page_start = page_num * 4096;
    let mut zero_count = 0;
    for j in page_start..(page_start + 4096) {
        if mmap[j] == 0 {
            zero_count += 1;
        }
    }
    println!("Page: {}, zero count: {}", page_num, zero_count)
}