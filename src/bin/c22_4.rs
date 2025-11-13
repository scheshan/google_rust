/***
你正在实现一个健康监控系统。作为其中的一部分，你需要对用户的健康统计数据进行追踪。

You’ll start with a stubbed function in an impl block as well as a User struct definition. Your goal is to implement the stubbed out method on the User struct defined in the impl block.

Copy the code below to https://play.rust-lang.org/ and fill in the missing method:
 */

// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]


#![allow(dead_code)]
pub struct User {
    name: String,
    age: u32,
    height: f32,
    visit_count: usize,
    last_blood_pressure: Option<(u32, u32)>,
}

pub struct Measurements {
    height: f32,
    blood_pressure: (u32, u32),
}

pub struct HealthReport<'a> {
    patient_name: &'a str,
    visit_count: u32,
    height_change: f32,
    blood_pressure_change: Option<(i32, i32)>,
}

impl User {
    pub fn new(name: String, age: u32, height: f32) -> Self {
        Self { name, age, height, visit_count: 0, last_blood_pressure: None }
    }

    pub fn visit_doctor(&mut self, measurements: Measurements) -> HealthReport {
        let height_change = measurements.height - self.height;
        let blood_pressure_change = match &self.last_blood_pressure {
            Some((x, y)) => Some((measurements.blood_pressure.0 as i32 - (*x as i32), measurements.blood_pressure.1 as i32 - (*y as i32))),
            None => None
        };
        let old_blood_pressure = self.last_blood_pressure.clone();

        self.visit_count += 1;
        self.height = measurements.height;
        self.last_blood_pressure = Some(measurements.blood_pressure);

        HealthReport {
            patient_name: &self.name,
            visit_count: self.visit_count as u32,
            height_change,
            blood_pressure_change,
        }
    }
}

fn main() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    println!("I'm {} and my age is {}", bob.name, bob.age);
}

#[test]
fn test_visit() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.visit_count, 0);
    let report =
        bob.visit_doctor(Measurements { height: 156.1, blood_pressure: (120, 80) });
    assert_eq!(report.patient_name, "Bob");
    assert_eq!(report.visit_count, 1);
    assert_eq!(report.blood_pressure_change, None);

    let report =
        bob.visit_doctor(Measurements { height: 156.1, blood_pressure: (115, 76) });

    assert_eq!(report.visit_count, 2);
    assert_eq!(report.blood_pressure_change, Some((-5, -4)));
}