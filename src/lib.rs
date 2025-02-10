use std::collections::HashSet;
#[derive(Debug, Default, Clone)]
pub struct RunOnce(HashSet<i32>);
impl RunOnce {
    pub fn run_once<F: FnOnce()>(&mut self, i: i32, stuff: F) {
        if !self.0.contains(&i) {
            self.0.insert(i);
            stuff()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut data = RunOnce::default();
        let mut value = 0;

        for _ in 0..5 {
            data.run_once(1, || {
                value += 1;
            });
        }

        assert_eq!(value, 1);
    }
}
