use leetcode::rs::leetcode_134::can_complete_circuit;

#[test]
fn case_1() {
    let gas = vec![1, 2, 3, 4, 5];
    let costs = vec![3, 4, 5, 1, 2];
    let resullt = can_complete_circuit(gas, costs);
    assert_eq!(resullt, 3);
}

#[test]
fn case_2() {
    let gas = vec![2, 3, 4];
    let costs = vec![3, 4, 3];
    let resullt = can_complete_circuit(gas, costs);
    assert_eq!(resullt, -1);
}
