fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<i32> = Vec::new();

    for &asteroid in asteroids.iter() {
        let mut alive = true;

        while alive {
            if asteroid > 0 || stack.is_empty() {
                // If the current asteroid is moving right or the stack is empty, push it onto the stack
                println!("Inital pushing: {}", asteroid);
                stack.push(asteroid);
                alive = false; // No collision will occur
            } else {
                // Current asteroid is moving left
                match stack.last() {
                    Some(&top) if top > 0 => {
                        println!("Top: {}, Current: {}", top, asteroid);
                        // Collision occurs
                        if top < -asteroid {
                            // Top asteroid explodes
                            println!("   Top asteroid explodes: {}", top);
                            stack.pop();
                        } else if top == -asteroid {
                            // Both asteroids explode
                            println!("   Both asteroids explode: {}", top);
                            stack.pop();
                            alive = false; // Current asteroid also explodes
                        } else {
                            // Current asteroid explodes
                            println!("   Current asteroid explodes: {}", asteroid);
                            alive = false;
                        }
                    }
                    _ => {
                        println!("No match, pushing: {}", asteroid);
                        // No collision, push the current asteroid onto the stack
                        stack.push(asteroid);
                        alive = false;
                    }
                }
            }
        }
    }

    stack
}

fn main() {
    let asteroids = vec![5, 10, -11];
    let result = asteroid_collision(asteroids);
    println!("{:?}", result); // Output: [5, 10]

    let asteroids2 = vec![8, -8];
    let result2 = asteroid_collision(asteroids2);
    println!("{:?}", result2); // Output: []

    let asteroids3 = vec![10, 2, -5];
    let result3 = asteroid_collision(asteroids3);
    println!("{:?}", result3); // Output: [10]

    let asteroids4 = vec![-2, -1, 1, 2];
    let result4 = asteroid_collision(asteroids4);
    println!("{:?}", result4); // Output: [-2, -1, 1, 2]
}