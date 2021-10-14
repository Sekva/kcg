use std::collections::VecDeque;

pub fn dq_vec<T: Copy>(dq: &VecDeque<T>) -> Vec<T> {
    let mut v = Vec::new();
    v.extend(dq.iter());
    v
}

pub fn iguais(a: &[usize], b: &[usize]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut a = Vec::from(a);
    let mut b = Vec::from(b);

    a.sort();
    b.sort();

    for (aa, bb) in a.iter().zip(b.iter()) {
        if aa != bb {
            return false;
        }
    }

    true
}
