#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

fn vec_sort(vec: &mut Vec<usize>) -> Vec<usize> {
    vec.sort();
    vec.to_vec()
}

fn float_vec_sort(vec: &mut Vec<f64>) -> Vec<f64> {
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    vec.to_vec()
}

fn struct_vec_sort<T>(vec: &mut Vec<T>) -> Vec<T>
where
    T: Ord + Clone,
{
    vec.sort();
    vec.to_vec()
}

fn main() {
    let mut vec = vec![5, 4, 2, 1, 3];
    vec_sort(&mut vec);
    println!("vec: {:?}", vec);

    let mut vec2 = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    float_vec_sort(&mut vec2);
    println!("vec2: {:?}", vec2);

    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    struct_vec_sort(&mut people);
    people.sort_by(|a, b| a.age.cmp(&b.age));

    println!("people: {:#?}", people);
}
