use std::fs;

fn main() {
    let contents =
        fs::read_to_string("src/13/data.txt").expect("Should have been able to read the file");
    let contents = &String::from(contents.trim());
    println!("Phase 1: {}", packet_scanners_phase_1(contents));
    println!("Phase 2: {}", packet_scanners_phase_2(contents));
}

#[derive(Debug)]
struct Layer {
    depth: i32,
    range: i32,
}

impl Layer {
    fn collides(&self, start_time: i32) -> bool {
        (self.depth + start_time) % ((self.range - 1) * 2) == 0
    }
    fn severity(&self) -> i32 {
        self.depth * self.range
    }
}

fn test_run(layers: &Vec<Layer>, start_time: i32) -> (i32, bool) {
    let mut severity = 0;
    let mut collision = false;
    for l in layers {
        if l.collides(start_time) {
            collision = true;
            severity += l.severity();
        }
    }
    (severity, collision)
}

fn packet_scanners_phase_1(input: &str) -> i32 {
    let layers: Vec<Layer> = input.lines().map(parse_layer).collect();
    test_run(&layers, 0).0
}

fn packet_scanners_phase_2(input: &str) -> i32 {
    let layers: Vec<Layer> = input.lines().map(parse_layer).collect();
    let mut start_time = 0;
    loop {
        if !test_run(&layers, start_time).1 {
            return start_time;
        }
        start_time += 1;
    }
}

fn parse_layer(input: &str) -> Layer {
    let l: Vec<i32> = input.split(": ").map(|v| v.parse().unwrap()).collect();
    Layer {
        depth: l[0],
        range: l[1],
    }
}

#[test]
fn test_parse_layer() {
    let layer_str = "123: 45";
    let layer = parse_layer(layer_str);
    assert_eq!(layer.depth, 123);
    assert_eq!(layer.range, 45);
}
