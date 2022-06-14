struct Container(i32, i32);

trait Containers {
    type A;
    type B;

    fn contains(&self, _: Self::A, _: Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Containers for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, number1: i32, number2: i32) -> bool {
        self.0 == number1 && self.1 == number2
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

fn difference<C: Containers>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {}: {}",
        number_1,
        number_2,
        container.contains(number_1, number_2)
    );
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}
