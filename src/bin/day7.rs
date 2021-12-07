// Advent of Code
// Day 7

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn part1() {
        let crabs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        let (pos, fuel) = min_fuel_position(&crabs);
        assert_eq!(2, pos);
        assert_eq!(37, fuel);
    }
}

#[cfg(test)]
mod test_positions {
    use super::*;

    #[test]
    fn position_1() {
        let positions = possible_positions(1, 5);
        assert_eq!(6, positions.len());

        assert_eq!(1, positions[0]);
        assert_eq!(0, positions[1]);
        assert_eq!(1, positions[2]);
        assert_eq!(2, positions[3]);
        assert_eq!(3, positions[4]);
        assert_eq!(4, positions[5]);
    }

    #[test]
    fn position_3() {
        let positions = possible_positions(3, 5);
        assert_eq!(6, positions.len());
        
        assert_eq!(3, positions[0]);
        assert_eq!(2, positions[1]);
        assert_eq!(1, positions[2]);
        assert_eq!(0, positions[3]);
        assert_eq!(1, positions[4]);
        assert_eq!(2, positions[5]);
    }

    #[test]
    fn position_5() {
        let positions = possible_positions(5, 5);
        assert_eq!(6, positions.len());
        
        assert_eq!(5, positions[0]);
        assert_eq!(4, positions[1]);
        assert_eq!(3, positions[2]);
        assert_eq!(2, positions[3]);
        assert_eq!(1, positions[4]);
        assert_eq!(0, positions[5]);
    }
}

fn possible_positions(pos: i32, max_pos: i32) -> Vec<i32> {
    let mut positions = Vec::new();
    for i in 0..(max_pos+1) {
        positions.push((pos - i).abs());
    }

    return positions;
}

fn min_fuel_position(crabs: &[i32]) -> (i32, i32) {
    return (0, 0);
}

fn main() {

}
