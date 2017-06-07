use super::*;

#[test]
fn sliding_windows_1() {
    fn test_window_correctness_with_storage(mut storage: Storage<u32>) {
        let expected: &[&[u32]] = &[&[0,1,2], &[1,2,3], &[2,3,4]];

        {
            let windowed_iter = Adaptor::new(0..5, &mut storage);
            let output: Vec<Vec<u32>> = windowed_iter.map(|x| From::from(&x[..])).collect();
            assert_eq!(output, expected);
        }
    }

    let auto_alloc:   Storage<u32> = Storage::optimized(&(0..5), 3);
    let middle_alloc: Storage<u32> = Storage::new(3);
    let small_alloc:  Storage<u32> = Storage::from_vec(vec![1u32;   1], 3);
    let exact_alloc:  Storage<u32> = Storage::from_vec(vec![1u32;   3], 3);
    let big_alloc:    Storage<u32> = Storage::from_vec(vec![1u32, 100], 3);

    test_window_correctness_with_storage(auto_alloc);
    test_window_correctness_with_storage(middle_alloc);
    test_window_correctness_with_storage(small_alloc);
    test_window_correctness_with_storage(exact_alloc);
    test_window_correctness_with_storage(big_alloc);
}

#[test]
fn sliding_windows_2() {
    let it = 0..5;
    let mut storage: Storage<u32> = Storage::optimized(&it, 3);
    let windowed_iter = Adaptor::new(it, &mut storage);

    for mut x in windowed_iter {
        x[1] = 0u32;
        assert_eq!(x[0], 0);
    }
}

#[test]
#[should_panic]
fn sliding_windows_3() {
    let it = 0..5;
    let mut storage: Storage<u32> = Storage::optimized(&it, 3);
    let mut windowed_iter = Adaptor::new(it, &mut storage);

    let _a = windowed_iter.next();
    let _b = windowed_iter.next();
}

#[test]
fn sliding_windows_4() {
    let mut storage1: Storage<u32> = Storage::new(3);
    let iter1 = Adaptor::new(0..5, &mut storage1);

    let mut storage2: Storage<u32> = Storage::new(5);
    let iter2 = Adaptor::new(0..5, &mut storage2);

    let mut storage3: Storage<u32> = Storage::new(6);
    let iter3 = Adaptor::new(0..5, &mut storage3);

    let mut storage4: Storage<u32> = Storage::new(6);
    let iter4 = Adaptor::new(42..42, &mut storage4);

    assert_eq!(iter1.size_hint(), (3, Some(3)));
    assert_eq!(iter2.size_hint(), (1, Some(1)));
    assert_eq!(iter3.size_hint(), (1, Some(1)));
    assert_eq!(iter4.size_hint(), (0, Some(0)));
}

#[test]
fn sliding_windows_5() {
    let range = 0..5;
    let mut storage: Storage<u32> = Storage::optimized(&range, 0);
    let mut iter = Adaptor::new(range, &mut storage);

    assert!(iter.next().is_none());
    assert!(iter.next().is_none());
}

#[test]
fn sliding_windows_6() {
    let range0 = 0..0;
    let range1 = 0..1;
    let range2 = 0..2;

    let storage0: Storage<u32> = Storage::optimized(&range0, 1);
    let storage0: Vec<u32> = storage0.into();
    assert_eq!(storage0.capacity(), 2);

    let storage1: Storage<u32> = Storage::optimized(&range1, 1);
    let storage1: Vec<u32> = storage1.into();
    assert_eq!(storage1.capacity(), 1);

    let storage2: Storage<u32> = Storage::optimized(&range2, 1);
    let storage2: Vec<u32> = storage2.into();
    assert_eq!(storage2.capacity(), 2);

    let storage3: Storage<u32> = Storage::optimized(&range2, 3);
    let storage3: Vec<u32> = storage3.into();
    assert_eq!(storage3.capacity(), 2);
}
