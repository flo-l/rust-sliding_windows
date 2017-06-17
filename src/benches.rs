use test::test::Bencher;
use super::*;

#[bench]
fn sliding_windows(b: &mut Bencher) {
    let num = 12;
    let data: &[u8] = &[num; 1024*1024];
    let mut storage: Storage<&u8> = Storage::new(10);

    b.iter(|| {
        let data = test::black_box(&data);
        let storage = test::black_box(&mut storage);

        let iter = Adaptor::new(data.iter(), storage);

        for window in iter {
            for x in window.iter() {
                let x: &u8 = test::black_box(x);
                assert_eq!(*x, num)
            }
        }
    });
}

#[bench]
fn slice_window(b: &mut Bencher) {
    let num = 12;
    let data: &[u8] = &[num; 1024*1024];

    b.iter(|| {
        let data = test::black_box(&data);
        let iter = data.windows(10);
        for window in iter {
            for x in window.iter() {
                let x: &u8 = test::black_box(x);
                assert_eq!(*x, num)
            }
        }
    });
}
