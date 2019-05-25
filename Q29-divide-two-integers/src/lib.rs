pub struct Solution;

const MIN_VALUE: i64 = i32::min_value() as i64;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let mut result = 0;
        // 转换为64位
        // 因为下面的处理需要用到绝对值
        // 如果是i32的min_value的绝对值会溢出
        let mut dividend = dividend as i64;
        let mut divisor = divisor as i64;
        let negative = (dividend ^ divisor) < 0;

        // 防溢出处理
        if dividend == MIN_VALUE && divisor == -1  {
            return i32::max_value();
        }

        dividend = dividend.abs();
        divisor = divisor.abs();

        for _i in 0..=31 {
            let i = 31 - _i;

            // 如果dividend / 2^i大于divider，
            // 那么说明dividend减去2^i个divider以后还可以被divider整除
            // 那么重复上面的过程并把得到的result累加即可
            // 直到最后divided本身就小于divisor以后，说明此时的dividend即为余数


            // 另一个需要注意的是，需要把i从大到小
            // 如果从小到大，无法避免i在满足稍大的时候同样满足稍小的i的这种情况
            if (dividend >> i) >= divisor {
                result += 1 << i;
                dividend -= divisor << i;
            }
        }

        if negative {
            -result as i32
        } else {
            result as i32
        }
    }
}