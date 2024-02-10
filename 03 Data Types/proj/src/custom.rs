struct Person {
    name: String,
    age: u8,
}

enum TraficLight {
    Red,
    Yellow,
    Green,
}

pub fn custom() {
    let _person = Person {
        name: String::from("Amal Murikkoli"),
        age: 25,
    };

    let light = TraficLight::Red;

    match light {
        TraficLight::Red => println!("Stop!"),
        TraficLight::Yellow => println!("Start!"),
        TraficLight::Green => println!("Go!"),
    }
}
