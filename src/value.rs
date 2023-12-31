#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    String(Box<String>),
    Float(f64),
}

impl Value {
    pub fn __add__(self, other: Self) -> Self {
        let a = self.as_int();
        let b = other.as_int();
        Value::Integer(a + b)
    }
    pub fn __sub__(self, other: Self) -> Self {
        let a = self.as_int();
        let b = other.as_int();
        Value::Integer(a - b)
    }
    pub fn __mul__(self, other: Self) -> Self {
        let a = self.as_int();
        let b = other.as_int();
        Value::Integer(a * b)
    }
    pub fn __div__(self, other: Self) -> Self {
        let a = self.as_int();
        let b = other.as_int();
        Value::Integer(a / b)
    }
    pub fn __gt__(self, other: Self) -> Self {
        let a = self.as_int();
        let b = other.as_int();
        Value::Integer((a > b) as i64)
    }
    pub fn __lt__(self, other: Self) -> Self {
        let a = self.as_int();
        let b = other.as_int();
        Value::Integer((a < b) as i64)
    }
    pub fn __et__(self, other: Self) -> Self {
        let a = self.as_int();
        let b = other.as_int();
        Value::Integer((a == b) as i64)
    }
    pub fn __ne__(self, other: Self) -> Self {
        let a = self.as_int();
        let b = other.as_int();
        Value::Integer((a != b) as i64)
    }
    pub fn __ge__(self, other: Self) -> Self {
        let a = self.as_int();
        let b = other.as_int();
        Value::Integer((a >= b) as i64)
    }
    pub fn __le__(self, other: Self) -> Self {
        let a = self.as_int();
        let b = other.as_int();
        Value::Integer((a <= b) as i64)
    }
    pub fn __repr__(&self) -> String {
        match self {
            Value::Integer(x) => format!("{}", x),
            Value::Float(f) => format!("{}", f),
            Value::String(s) => *s.clone(),
        }
    }
    pub fn as_bool(self) -> bool {
        match self {
            Value::Integer(v) => v > 0,
            _ => panic!("Expected integer"),
        }
    }
    pub fn as_int(self) -> i64 {
        match self {
            Value::Integer(v) => v,
            _ => panic!("Expected integer"),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn value_size() {
        assert_eq!(std::mem::size_of::<Value>() * 8, 128);
    }
}
