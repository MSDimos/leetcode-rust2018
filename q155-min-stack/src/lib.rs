struct MinStack {
    min_value: Vec<i32>,
    inner_stack: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            inner_stack: Vec::new(),
            min_value: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.inner_stack.push(x);

        if let Some(mv) = self.min_value.pop() {
            if mv >= x {
                self.min_value.push(mv);
                self.min_value.push(x);
            } else {
                self.min_value.push(mv);
            }
        } else {
            self.min_value.push(x);
        }
    }

    fn pop(&mut self) {
        if let Some(x_tmp) = self.inner_stack.pop() {
            if let Some(m_tmp) = self.min_value.pop() {
                if x_tmp != m_tmp {
                    self.min_value.push(m_tmp);
                }
            }
        }
    }

    fn top(&self) -> i32 {
        if self.inner_stack.is_empty() {
            -1
        } else {
            self.inner_stack[self.inner_stack.len() - 1]
        }
    }

    fn get_min(&self) -> i32 {
        if let Some(min) = self.min_value.last() {
            *min
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::MinStack;

    #[test]
    fn it_works() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }
}
