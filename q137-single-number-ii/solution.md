### states
- 00, initial state
- 01, get a b'1'
- 02, get two b'1's

### state transition graph
```txt
(00) -1-> (01) -1-> (10) ---
 ^                          |
 |                          | 
 --------------1-------------
if get 0, state will keep current state but will not transfer.

```

### state trasition table

According to the state trasition graph, we can fill the state transition table easily.

| x(input) | Q<sub>1</sub><sup>n</sup>Q<sub>0</sub><sup>n</sup>(current state) | Q<sub>1</sub><sup>n+1</sup>Q<sub>0</sub><sup>n+1</sup>(next state) |
| :------: | :----------------------------------------------------------: | :----------------------------------------------------------: |
|    0     |                              00                              |                              00                              |
|    0     |                              01                              |                              01                              |
|    0     |                              10                              |                              10                              |
|    1     |                              00                              |                              01                              |
|    1     |                              01                              |                              10                              |
|    1     |                              10                              |                              00                              |

### Karnaugh map

|      |  00  |  01  |  10  |
| :--: | :--: | :--: | :--: |
|  0   |  0   |  1   |  0   |
|  1   |  1   |  0   |  0   |

<center>Q<sub>0</sub><sup>n+1</sup> Karnaugh map</center>

|      |  00  |  01  |  10  |
| :--: | :--: | :--: | :--: |
|  0   |  0   |  0   |  1   |
|  1   |  0   |  1   |  0   |

<center>Q<sub>1</sub><sup>n+1</sup> Karnaugh map</center>

### state transition equation

$$
{Q_{0}}^{n+1} = \overline{X}\overline{{Q_{1}}^{n}}{Q_{0}}^{n}+X\overline{{Q_{1}}^{n}}\overline{{Q_{0}}^{n}}
$$

$$
{Q_{1}}^{n+1} = \overline{X}{Q_{1}}^{n}\overline{{Q_{0}}^{n}}+X\overline{{Q_{1}}^{n}}{Q_{0}}^{n}
$$

### code

So we can program now. I like to use `Rust`, easy to understand.

```rust
pub struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut a = 0;
        let mut b = 0;

        for num in nums {
            b = (b ^ num) & !a;
            a = (a ^ num) & !b;
        }

        return b;
    }

    pub fn single_number_s2(nums: Vec<i32>) -> i32 {
        let mut q0 = 0;
        let mut q1 = 0;

        for x in nums {
            let q0_next = ((!x) & (!q1) & q0) | (x & (!q1) & (!q0));
            let q1_next = ((!x) & q1 & (!q0)) | (x & (!q1) & q0);

            q0 = q0_next;
            q1 = q1_next;
        }

        // If try to find out character which appears twice, return q1.
        // Why? Looking column named next state of penultimate row 
        // in state transition table above, you can find out the reason.
        q0
    }
}

```

