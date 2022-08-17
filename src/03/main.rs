use std::fs;

struct Turtle<'a> {
    position: (usize, usize),
    direction: (isize, isize),
    field: &'a mut Vec<Vec<u32>>
}

impl Turtle<'_> {
    fn move_forward(&mut self) {
        self.position.0 = (self.position.0 as isize + self.direction.0) as usize;
        self.position.1 = (self.position.1 as isize + self.direction.1) as usize;
    }

    fn turn_left(&mut self) {
        match self.direction {
            (0, 1) => self.direction = (1, 0),
            (1, 0) => self.direction = (0, -1),
            (0, -1) => self.direction = (-1, 0),
            (-1, 0) => self.direction = (0, 1),
            _ => panic!("help")
        };
    }

    fn scan_left(&self) -> u32 {
        match self.direction {
            (0, 1) => self.field[self.position.0 + 1][self.position.1],
            (1, 0) => self.field[self.position.0][self.position.1 - 1],
            (0, -1) => self.field[self.position.0 - 1][self.position.1],
            (-1, 0) => self.field[self.position.0][self.position.1 + 1],
            _ => panic!("help")
        }
    }

    fn sum_surrounding(&self) -> u32 {
        let x = self.position.0;
        let y = self.position.1;
        self.field[x-1] [y-1]   +
        self.field[x-1] [y]     +
        self.field[x-1] [y+1]   +
        self.field[x]   [y-1]   +
        self.field[x]   [y+1]   +
        self.field[x+1] [y-1]   +
        self.field[x+1] [y]     +
        self.field[x+1] [y+1]
    }

    fn tick(&mut self) {
        self.field[self.position.0][self.position.1] = self.sum_surrounding();
        self.move_forward();
        if self.scan_left() == 0 { self.turn_left(); }
    }

    fn _print_field(&self) {
        println!("{:?}", self.field);
    }
}

fn main() {
    let contents = fs::read_to_string("src/03/data.txt")
        .expect("Should have been able to read the file");
    let contents = &String::from(contents.trim());
    println!("Phase 1: {}", spiral_memory_phase_1(contents));
    println!("Phase 2: {}", spiral_memory_phase_2(contents));
}

fn spiral_memory_phase_2(input: &String) -> u32 {
    let address: u32 = input.parse().unwrap();
    let l = layer(address) + 1; // protect from off-by-one errors
    let dim = layer_dim(l) as usize;
    let mut field: Vec<Vec<u32>> = Vec::from([]);
    for _ in 0..dim {
        field.push(vec![0; dim]);
    }
    field[dim / 2][dim / 2] = 1;
    let mut t = Turtle {
        position: (dim / 2, dim / 2 + 1),
        direction: (1, 0),
        field: &mut field,
    };
    while t.sum_surrounding() < address {
        t.tick();
    }
    t.sum_surrounding()
}

fn spiral_memory_phase_1(input: &String) -> u32 {
    let address: u32 = input.parse().unwrap();
    let l = layer(address);
    let diags = layer_diags(l);
    if address < diags.0 {
        l + address.abs_diff(layer_axes(l).0) - 1
    } else if address < diags.1 {
        l + address.abs_diff(layer_axes(l).1) - 1
    } else if address < diags.2 {
        l + address.abs_diff(layer_axes(l).2) - 1
    } else if address < diags.3 {
        l + address.abs_diff(layer_axes(l).3) - 1
    } else {
        0
    }
}

fn layer_dim(l: u32) -> u32 { 2 * l - 1 }
fn layer_footprint(l: u32) -> u32 { layer_dim(l) * layer_dim(l) }

fn layer_diags(l: u32) -> (u32, u32, u32, u32) {
    let max = layer_footprint(l);
    let dim = layer_dim(l);
    (
        max,
        max - (dim - 1),
        max - (2 * (dim - 1)),
        max - (3 * (dim - 1)),
    )
}

fn layer_axes(l: u32) -> (u32, u32, u32, u32) {
    let max = layer_footprint(l);
    let dim = layer_dim(l) - 1;
    let half_dim = dim / 2;
    (
        max - half_dim,
        max - dim - half_dim,
        max - (2 * dim) - half_dim,
        max - (3 * dim) - half_dim,
    )
}

fn layer(address: u32) -> u32 {
    let mut l = 1;
    while address > layer_footprint(l) {
        l += 1;
    }
    l
}

#[test]
fn test_layers() {
    assert_eq!(layer(1), 1);
    for i in 2..9 {
        assert_eq!(layer(i), 2);
    }
    for i in 10..25 {
        assert_eq!(layer(i), 3);
    }
    for i in 26..49 {
        assert_eq!(layer(i), 4);
    }
    assert_eq!(layer(50), 5);
}

#[test]
fn test_layer_footprint() {
    assert_eq!(layer_footprint(1), 1);
    assert_eq!(layer_footprint(2), 9);
    assert_eq!(layer_footprint(3), 25);
    assert_eq!(layer_footprint(4), 49);
    assert_eq!(layer_footprint(5), 81);
}

#[test]
fn test_layer_axes() {
    assert_eq!(layer_axes(1), (1, 1, 1, 1));
    assert_eq!(layer_axes(2), (8, 6, 4, 2));
    assert_eq!(layer_axes(3), (23, 19, 15, 11));
}

#[test]
fn test_layer_diags() {
    assert_eq!(layer_diags(1), (1, 1, 1, 1));
    assert_eq!(layer_diags(2), (9, 7, 5, 3));
    assert_eq!(layer_diags(3), (25, 21, 17, 13));
}

