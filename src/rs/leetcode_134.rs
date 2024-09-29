pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let length = gas.len();
    let mut i: usize = 0;
    while i < length {
        let mut sum_gas = 0;
        let mut sum_cost = 0;
        let mut current = 0;
        while current < length {
            let j = (i + current) % length;
            sum_gas += gas[j];
            sum_cost += cost[j];
            if sum_cost > sum_gas {
                break;
            } else {
                current += 1;
            }
        }
        if current == length {
            return i as i32;
        } else {
            i = i + current + 1;
        }
    }
    return -1;
}
