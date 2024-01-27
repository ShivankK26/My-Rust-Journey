fn main() {
    let int_container = Container { value: 42 };
    let str_container = Container { value: "Simon" };

    println!("{}", int_container.value);
    println!("{}", str_container.value);

    let success_result: MagicalResult<i32, String> = MagicalResult::Success(42);
    let failure_result: MagicalResult<i32, String> = MagicalResult::Failure("Failed!".to_string());

    match success_result {
        MagicalResult::Success(value) => println!("We got success {}", value),
        MagicalResult::Failure(value) => println!("We got failure {}", value),
    }

    match failure_result {
        MagicalResult::Success(value) => println!("We got success {}", value),
        MagicalResult::Failure(value) => println!("We got failure {}", value),
    }

    let mut honda = ElectricCar { battery_level: 42 };
    let mut ford = GasCar { gas_level: 20 };

    honda.refuel(58);
    ford.refuel(0.2);
}

struct Container<T> {
    value: T,
}

enum MagicalResult<T, E> {
    Success(T),
    Failure(E),
}

// Practicing a Nice Question
trait Vehicle {
    type Fuel;

    fn refuel(&mut self, fuel: Self::Fuel);
}

struct ElectricCar {
    battery_level: u32,
}

struct GasCar {
    gas_level: u32,
}

impl Vehicle for ElectricCar {
    type Fuel = u32;

    fn refuel(&mut self, charge: Self::Fuel) {
        self.battery_level += charge;
        println!("Battery charged to {}%", self.battery_level);
    }
}

impl Vehicle for GasCar {
    type Fuel = u32;

    fn refuel(&mut self, gas: Self::Fuel) {
        self.gas_level += (gas * 100.0) as u32;
        println!("Gas tank filled to {}%", self.gas_level);
    }
}
