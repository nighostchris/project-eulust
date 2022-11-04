// Lattice paths

// from starting point 0
// -> take either path left or right
// -> mark the current node as visited
// -> if reaching [num, num] then return 1
// -> if no places to go then return 0

// function parameter => (current x, current y, visited node map)

// https://projecteuler.net/problem=15

fn path_finder(x: usize, y: usize, visited_map: Vec<Vec<bool>>, grid_size: usize) -> u64 {
    if x > grid_size || y > grid_size {
        return 0;
    }

    if x == grid_size && y == grid_size {
        return 1;
    }

    let mut new_visited_map = visited_map.clone();
    new_visited_map[x][y] = true;
    let left_paths = path_finder(x + 1, y, new_visited_map.clone(), grid_size);
    let right_paths = path_finder(x, y + 1, new_visited_map.clone(), grid_size);

    return left_paths + right_paths;
}

fn combination(n: u64, k: u64) -> u64 {
    (1..k + 1).fold(1, |acc, num| acc * (n - num + 1) / num)
}

pub fn lattice_paths(num: u64) {
    // Slow Method
    // let initial_visited_map = vec![vec![false; num + 1]; num + 1];
    // let sum = path_finder(0, 0, initial_visited_map, num);

    // Fast Method - Combination
    // Out of 2n different coordinates available for next step
    // We will choose n coordinates
    let sum = combination(num * 2, num);

    println!("Total number of routes for a {}x{} grid: {}", num, num, sum);
}
