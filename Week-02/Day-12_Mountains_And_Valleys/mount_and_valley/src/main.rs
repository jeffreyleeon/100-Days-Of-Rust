#[derive(Debug, PartialEq)]
enum LandscapeType {
    Mountain,
    Valley,
    None,
}

fn mount_or_valley(arr: Vec<i32>) -> LandscapeType {
    if arr.len() < 3 {
        return LandscapeType::None;
    }
    let mut left_pointer = 0;
    let mut right_pointer = arr.len() - 1;
    let (mut left_trend, mut right_trend) = (LandscapeType::None, LandscapeType::None);
    let (mut left_end, mut right_end) = (false, false);
    while left_pointer < right_pointer && !(left_end && right_end) {
        if !left_end {
            if arr[left_pointer] < arr[left_pointer + 1] {
                if left_trend == LandscapeType::Valley {
                    left_end = true;
                    continue;
                }
                left_trend = LandscapeType::Mountain;
            } else if arr[left_pointer] > arr[left_pointer + 1] {
                if left_trend == LandscapeType::Mountain {
                    left_end = true;
                    continue;
                }
                left_trend = LandscapeType::Valley;
            }
        }
        if !right_end {
            if arr[right_pointer] < arr[right_pointer - 1] {
                if right_trend == LandscapeType::Valley {
                    right_end = true;
                    continue;
                }
                right_trend = LandscapeType::Mountain;
            } else if arr[right_pointer] > arr[right_pointer - 1] {
                if right_trend == LandscapeType::Mountain {
                    right_end = true;
                    continue;
                }
                right_trend = LandscapeType::Valley;
            }
        }
        left_pointer += 1;
        right_pointer -= 1;
    }
    if left_pointer < right_pointer {
        return LandscapeType::None;
    }
    println!("Left and right trend at the end: {:?} {:?}", left_trend, right_trend);
    if left_trend == LandscapeType::Mountain && right_trend == LandscapeType::Mountain {
        return LandscapeType::Mountain;
    } else if left_trend == LandscapeType::Valley && right_trend == LandscapeType::Valley {
        return LandscapeType::Valley;
    }
    LandscapeType::None
}

fn main() {
    let arr: Vec<i32> = vec![1, 2, 3, 2, 1];
    let result: LandscapeType = mount_or_valley(arr);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mount_or_valley() {
        assert_eq!(mount_or_valley(vec![1, 2, 3, 2, 1]), LandscapeType::Mountain);
        assert_eq!(mount_or_valley(vec![3, 2, 1, 2, 3]), LandscapeType::Valley);
        assert_eq!(mount_or_valley(vec![1, 2, 3, 4, 5]), LandscapeType::None);
        assert_eq!(mount_or_valley(vec![5, 4, 3, 2, 1]), LandscapeType::None);
        assert_eq!(mount_or_valley(vec![1, 2, 3, 2, 3]), LandscapeType::None);
        assert_eq!(mount_or_valley(vec![1, 2, 3, 2]), LandscapeType::Mountain);
        assert_eq!(mount_or_valley(vec![1, 2, 3]), LandscapeType::None);
        assert_eq!(mount_or_valley(vec![1, 2]), LandscapeType::None);
        assert_eq!(mount_or_valley(vec![1]), LandscapeType::None);
        assert_eq!(mount_or_valley(vec![]), LandscapeType::None);

        assert_eq!(mount_or_valley(vec![1, 3, 5, 4, 3, 2]), LandscapeType::Mountain);
        assert_eq!(mount_or_valley(vec![-1, 0, -1]), LandscapeType::Mountain);
        assert_eq!(mount_or_valley(vec![-1, -1, 0, -1, -1]), LandscapeType::Mountain);

        assert_eq!(mount_or_valley(vec![10, 9, 8, 7, 2, 3, 4, 5]), LandscapeType::Valley);
        assert_eq!(mount_or_valley(vec![350, 100, 200, 400, 700]), LandscapeType::Valley);

        assert_eq!(mount_or_valley(vec![1, 2, 3, 2, 4, 1]), LandscapeType::None);
        assert_eq!(mount_or_valley(vec![5, 4, 3, 2, 1]), LandscapeType::None);
        assert_eq!(mount_or_valley(vec![0, -1, -1, 0, -1, -1]), LandscapeType::None);
    }
}