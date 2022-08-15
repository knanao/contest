fn main() {
    let mut input = vec![4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    merge_sort(&mut input);
    println!("{:?}", input);
}

fn merge_sort<T: Copy + Ord>(x: &mut [T]) {
    let (n, mid) = (x.len(), x.len() / 2);
    if n < 2 {
        return;
    }

    merge_sort(&mut x[0..mid]);
    merge_sort(&mut x[mid..n]);

    let mut y: Vec<T> = x.to_vec();
    merge(&x[0..mid], &x[mid..n], &mut y[..]);
    x.copy_from_slice(&y)
}

fn merge<T: Copy + PartialOrd>(left: &[T], right: &[T], y: &mut [T]) {
    let (mut l, mut r, mut i) = (0, 0, 0);

    while l < left.len() && r < right.len() {
        if left[l] <= right[r] {
            y[i] = left[l];
            l += 1;
        } else {
            y[i] = right[r];
            r += 1;
        }
        i += 1;
    }

    if l < left.len() {
        y[i..].copy_from_slice(&left[l..]);
    }

    if r < right.len() {
        y[i..].copy_from_slice(&right[r..]);
    }
}
