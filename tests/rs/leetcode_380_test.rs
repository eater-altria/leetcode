use leetcode::rs::leetcode_380::RandomizedSet;

#[test]
fn case_1() {
    let mut randomized_set = RandomizedSet::new();

    let result = randomized_set.insert(1);
    assert_eq!(result, true);

    let result = randomized_set.remove(2);
    assert_eq!(result, false);

    let result = randomized_set.insert(2);
    assert_eq!(result, true);

    let result = randomized_set.get_random();
    assert!(result == 1 || result == 2);

    let result = randomized_set.remove(1);
    assert_eq!(result, true);

    let result = randomized_set.insert(2);
    assert_eq!(result, false);

    let result = randomized_set.get_random();
    assert_eq!(result, 2);
}

#[test]
fn case_2() {
    let mut randomized_set = RandomizedSet::new();

    let result = randomized_set.remove(0);
    assert_eq!(result, false);

    let result = randomized_set.remove(0);
    assert_eq!(result, false);

    let result = randomized_set.insert(0);
    assert_eq!(result, true);

    let result = randomized_set.get_random();
    assert_eq!(result, 0);

    let result = randomized_set.remove(0);
    assert_eq!(result, true);

    let result = randomized_set.insert(0);
    assert_eq!(result, true);
}

#[test]
fn case_3() {
    let mut randomized_set = RandomizedSet::new();

    let result = randomized_set.insert(1);
    assert_eq!(result, true);

    let result = randomized_set.insert(10);
    assert_eq!(result, true);

    let result = randomized_set.insert(20);
    assert_eq!(result, true);

    let result = randomized_set.insert(30);
    assert_eq!(result, true);

    let result = randomized_set.get_random();
    assert!(result == 1 || result == 10 || result == 20 || result == 30);

    let result = randomized_set.get_random();
    assert!(result == 1 || result == 10 || result == 20 || result == 30);

    let result = randomized_set.get_random();
    assert!(result == 1 || result == 10 || result == 20 || result == 30);

    let result = randomized_set.get_random();
    assert!(result == 1 || result == 10 || result == 20 || result == 30);

    let result = randomized_set.get_random();
    assert!(result == 1 || result == 10 || result == 20 || result == 30);
}
