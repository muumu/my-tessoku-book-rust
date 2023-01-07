use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }
    let mut L = vec![A[0]; 1];
    L.reserve(N);
    for i in 0..N {
        if A[i] > L[L.len() - 1] {
            L.push(A[i]);
        } else {
            let p = lower_bound(&L, A[i]);
            L[p] = A[i];
        }
    }
    println!("{}", L.len());
}

fn lower_bound(v: &Vec<usize>, elem: usize) -> usize {
    let mut low = 0;
    let mut high = v.len() - 1;
    while low <= high {
        let mid = (low + high) / 2;
        if v[mid] == elem {
            return mid;
        } else if v[mid] < elem {
            if mid == v.len() - 1 || elem <= v[mid + 1] {
                return mid + 1;
            }
            low = mid + 1;
        } else if mid == 0 {
            return 0;
        } else {
            high = mid - 1;
        }
    }
    return 0;
}
