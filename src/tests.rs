use super::*;

#[test]
fn sliding_windows_1() {
    fn test_window_correctness_with_storage(mut storage: SlidingWindowStorage<u32>) {
        let expected: &[&[u32]] = &[&[0,1,2], &[1,2,3], &[2,3,4]];

        {
            let windowed_iter = SlidingWindowAdaptor::new(0..5, &mut storage);
            let output: Vec<Vec<u32>> = windowed_iter.map(|x| From::from(&x[..])).collect();
            assert_eq!(output, expected);
        }

        let storage_vec: Vec<u32> = storage.into();
        assert_eq!(*expected.last().unwrap(), &*storage_vec);
    }

    let auto_alloc:  SlidingWindowStorage<u32> = SlidingWindowStorage::new(3);
    let small_alloc: SlidingWindowStorage<u32> = SlidingWindowStorage::from_vec(vec![1u32;   1], 3);
    let exact_alloc: SlidingWindowStorage<u32> = SlidingWindowStorage::from_vec(vec![1u32;   3], 3);
    let big_alloc:   SlidingWindowStorage<u32> = SlidingWindowStorage::from_vec(vec![1u32, 100], 3);

    test_window_correctness_with_storage(auto_alloc);
    test_window_correctness_with_storage(small_alloc);
    test_window_correctness_with_storage(exact_alloc);
    test_window_correctness_with_storage(big_alloc);
}

#[test]
fn sliding_windows_2() {
    let it = 0..5;
    let mut storage: SlidingWindowStorage<u32> = SlidingWindowStorage::new(3);
    let windowed_iter = SlidingWindowAdaptor::new(it, &mut storage);

    for mut x in windowed_iter {
        x[1] = 0u32;
        assert_eq!(x[0], 0);
    }
}

#[test]
#[should_panic]
fn sliding_windows_3() {
    let it = 0..5;
    let mut storage: SlidingWindowStorage<u32> = SlidingWindowStorage::new(3);
    let mut windowed_iter = SlidingWindowAdaptor::new(it, &mut storage);

    let _a = windowed_iter.next();
    let _b = windowed_iter.next();
}
