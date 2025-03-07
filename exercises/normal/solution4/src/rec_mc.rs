pub fn dp_rec_mc(amount: u32) -> u32 {
    // Literal
    let denominations = vec![1, 2, 5, 10, 20, 30, 50, 100];
    // Create an array to store the minimum number of change notes for each amount
    let mut min_coins = vec![u32::MAX; (amount + 1) as usize];
    // 0 Yuan
    min_coins[0] = 0;
    // Dynamic programming calculates the minimum number of change notes
    for i in 1..=amount as usize {
        for &denom in &denominations {
            if denom as u32 <= i as u32 {
                min_coins[i] = min_coins[i].min(min_coins[i - denom as usize] + 1);
            }
        }
    }
    // If the amount cannot be composed of a given denomination of paper money
    if min_coins[amount as usize] == u32::MAX {
        0
    } else {
        min_coins[amount as usize]
    }

}
