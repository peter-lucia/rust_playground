
pub struct Solution {

}

pub trait MyBuySellStock {
    /*
    pub is implied in traits
     */
    fn max_profit(&self, prices: Vec<i32>) -> i32;
}


impl MyBuySellStock for Solution {

    fn max_profit(&self, prices: Vec<i32>) -> i32 {
        let mut maxprofit = 0;
        for i in 1..prices.len() {
            if prices[i] > prices[i-1] {
                maxprofit += prices[i] - prices[i - 1]
            }
        }
        return maxprofit;
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;
    use super::MyBuySellStock;

    #[test]
    fn test_basic() {
        let solution = Solution{};
        let v = vec![7,1,5,3,6,4];
        assert_eq!(solution.max_profit(v), 7);
    }
}