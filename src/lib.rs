extern crate rand;
extern crate rayon;

// -------------------------------------------------------------
// In-place Quicksort
// -------------------------------------------------------------

fn random(upper: usize) -> usize {
    use rand::Rng;
    rand::weak_rng().gen_range(0, upper)
}

pub fn quicksort<T:PartialOrd+Send+Clone>(v: &mut [T]) {
    if v.len() <= 1 { return; }
    let mid = hoare_partition(v);
    let (lo, hi) = v.split_at_mut(mid);
    rayon::join(|| quicksort(lo),
                || quicksort(hi));
}

#[allow(dead_code)]
fn lomuto_partition<T:PartialOrd>(v: &mut [T]) -> usize {
    // see https://en.wikipedia.org/wiki/Quicksort#Lomuto_partition_scheme
    let pivot = v.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if v[j] <= v[pivot] {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, pivot);
    i
}

#[allow(dead_code)]
fn hoare_partition<T:PartialOrd+Clone>(v: &mut [T]) -> usize {
    // see https://en.wikipedia.org/wiki/Quicksort#Hoare_partition_scheme
    let r = random(v.len());
    let pivot = v[r].clone();
    let mut left = 0;
    let mut right = v.len()-1;
    loop {
        while v[left] < pivot  { left += 1 }
        while v[right] > pivot { right -= 1 }
        if left < right {
            v.swap(left, right);
            left += 1;
            right -= 1;
        } else {
            return right+1;
        }
    }
}

#[cfg(test)]
pub mod test {
    use rand::{self, Rng};

    use super::quicksort;

    fn sorted<T: PartialOrd>(arr: & [T]) -> bool {
        if arr.len() <= 1 { return true; }
        for i in 0 .. arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                return false;
            }
        }
        return true;
    }

    #[test]
    pub fn empty_vec() {
        let mut arr : Vec<u8> = vec![];
        quicksort(&mut arr);
        assert!(sorted(&arr));
    }

    #[test]
    pub fn all_equal() {
        let mut arr1 : Vec<u8> = vec![0, 0, 0, 0];     // even num of elems
        let mut arr2 : Vec<u8> = vec![11, 11, 11, 11]; // odd num of elems
        quicksort(&mut arr1);
        assert!(sorted(&arr1));
        quicksort(&mut arr2);
        assert!(sorted(&arr2));

    }

    #[test]
    pub fn reversed_order() {
        let mut arr : Vec<u8> = vec![5, 4, 3, 2, 2, 1];
        quicksort(&mut arr);
        assert!(sorted(&arr));
    }

    #[test]
    pub fn random_lengths() {
        let mut rng = rand::weak_rng();
        let max_len = 5000;
        for _ in 0 .. 100 {
            let r: usize = rng.gen();
            let len = (r % max_len) + 1;
            let mut nums: Vec<_> = rng.gen_iter::<i32>().take(len).collect();
            quicksort(&mut nums);
            assert!(sorted(&nums));
        }
    }

    #[test]
    pub fn million_elements() {
        let mut rng = rand::weak_rng();
        let mut nums: Vec<_> = rng.gen_iter::<f64>().take(1_000_000).collect();
        quicksort(&mut nums);
        assert!(sorted(&nums));
    }

}
