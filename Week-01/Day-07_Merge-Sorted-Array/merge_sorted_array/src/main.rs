fn merge(nums1: &mut Vec<i32>, m: usize, nums2: &[i32], n: usize) {
    let mut i = m as isize - 1; // Pointer for nums1
    let mut j = n as isize - 1; // Pointer for nums2
    let mut k = (m + n) as isize - 1; // Pointer for the merged array

    // Merge in reverse order
    while i >= 0 || j >= 0 {
        if i < 0 {
            // If nums1 is exhausted, copy remaining elements from nums2
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
        } else if j < 0 {
            // If nums2 is exhausted, copy remaining elements from nums1
            nums1[k as usize] = nums1[i as usize];
            i -= 1;
        } else if nums1[i as usize] > nums2[j as usize] {
            // If the current element in nums1 is greater, copy it
            nums1[k as usize] = nums1[i as usize];
            i -= 1;
        } else {
            // Otherwise, copy the current element from nums2
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
        }
        // Move the pointer to the next position
        k -= 1;
    }
}

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let nums2 = vec![2, 5, 6];
    let m = 3;
    let n = 3;

    merge(&mut nums1, m, &nums2, n);
    println!("{:?}", nums1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let mut nums1 = vec![4, 5, 6, 0, 0, 0];
        let nums2 = vec![1, 2, 3];
        let m = 3;
        let n = 3;

        merge(&mut nums1, m, &nums2, n);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_merge_empty_nums2() {
        let mut nums1 = vec![1, 2, 3];
        let nums2 = vec![];
        let m = 3;
        let n = 0;

        merge(&mut nums1, m, &nums2, n);
        assert_eq!(nums1, vec![1, 2, 3]);
    }

    #[test]
    fn test_merge_empty_nums1() {
        let mut nums1 = vec![0, 0, 0];
        let nums2 = vec![1, 2, 3];
        let m = 0;
        let n = 3;

        merge(&mut nums1, m, &nums2, n);
        assert_eq!(nums1, vec![1, 2, 3]);
    }

}