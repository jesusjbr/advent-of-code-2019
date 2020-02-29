use std::collections::HashMap;

fn get_orbits(orbit_map: Vec<&str>) -> HashMap<&str, &str> {
    //Childs are keys and parents are values
    let orbits: HashMap<&str, &str> = orbit_map
        .iter()
        .map(|text| {
            let mut it = text.split(')').rev();
            (it.next().unwrap(), it.next().unwrap())
        })
        .collect();
    orbits
}

fn total_orbits(orbit_map: Vec<&str>) -> u32 {
    let orbits = get_orbits(orbit_map);
    let mut total: u32 = 0;
    for spatial_object in orbits.clone() {
        //Calculate path weight for this leaf
        let mut path_weight = 0;
        let mut node = spatial_object.0;
        while node != "COM" {
            path_weight += 1;
            node = orbits.get(&node).unwrap();
        }
        total += path_weight;
    }
    total
}

//Given a leaf returns the path to reach C
fn path_to_com<'a>(orbits: &HashMap<&'a str, &'a str>, leaf: &'a str) -> Vec<&'a str> {
    let mut path: Vec<&str> = Vec::new();
    //Calculate path weight for this leaf
    let mut node = leaf;
    while node != "COM" {
        path.push(&node);
        node = orbits.get(&node).unwrap();
    }
    path.push("COM");
    path
}

//Given 2 leafs return the first node in which both intersects
fn intersection<'a>(
    orbits: &HashMap<&'a str, &'a str>,
    first_leaf: &'a str,
    second_leaf: &'a str,
) -> &'a str {
    let first_path: Vec<_> = path_to_com(&orbits, first_leaf);
    let second_path: Vec<_> = path_to_com(&orbits, second_leaf);

    for node in &first_path {
        for another in &second_path {
            if node == another {
                return node;
            }
        }
    }
    //Unreachable under normal circumstances
    return "COM";
}

fn orbital_transfers(orbit_map: Vec<&str>, first_leaf: &str, second_leaf: &str) -> usize {
    let orbits = get_orbits(orbit_map);
    let first_path: Vec<_> = path_to_com(&orbits, first_leaf);
    let second_path: Vec<_> = path_to_com(&orbits, second_leaf);
    let intersection = intersection(&orbits, first_leaf, second_leaf);
    //When counting the steps doesnt must count himself (YOU and SAN  -1)
    let distance = first_path
        .iter()
        .take_while(|s| **s != intersection)
        .count()
        - 1;
    let another_distance = second_path
        .iter()
        .take_while(|s| **s != intersection)
        .count()
        - 1;
    distance + another_distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let example = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
        ];
        assert_eq!(total_orbits(example), 42);
    }

    #[test]
    fn test_part1() {
        let input: Vec<_> = include_str!("../input/day6.txt").lines().collect();
        assert_eq!(total_orbits(input), 300598);
    }

    #[test]
    fn example_part2() {
        let example = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN",
        ];
        assert_eq!(orbital_transfers(example, "YOU", "SAN"), 4);
    }

    #[test]
    fn test_part2() {
        let input: Vec<_> = include_str!("../input/day6.txt").lines().collect();
        assert_eq!(orbital_transfers(input, "YOU", "SAN"), 520);
    }
}
