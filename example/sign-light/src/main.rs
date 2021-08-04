mod lib;
#[derive(Debug)]
enum LightType {
    Red,
    Yellow,
    Green,
}

struct Light {
    color: LightType,
    time: u32,
}

struct Sign {
    orientation: String,
    lights: Vec<Light>,
}



impl Sign {
    fn duration(&self, color: LightType) -> u32 {
        println!("{:?}", color);
        3
    }
}


fn main() {
    println!("Hello, world!");

    let red_light = Light { color: LightType::Red, time: 30};
    let yellow_light = Light { color: LightType::Yellow, time: 5};
    let green_light = Light { color: LightType::Green, time: 30};

    let left_sign = Sign {
        orientation: String::from("left"),
        lights: vec![red_light, yellow_light, green_light]
    };
    for i in &left_sign.lights {
        println!("{:?} {}", i.color, i.time);
    }
    lib::function();
    println!("{}", left_sign.orientation);
    left_sign.duration(LightType::Yellow);
    
}