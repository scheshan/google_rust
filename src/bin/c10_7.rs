/***
我们将创建一个数据结构来表示电梯控制系统中的事件。您可以自行定义用于构造各种事件的类型和函数。使用 #[derive(Debug)] 以允许通过 {:?} 设置类型格式。

This exercise only requires creating and populating data structures so that main runs without errors. The next part of the course will cover getting data out of these structures.
 */

#[derive(Debug)]
/// An event in the elevator system that the controller must react to.
enum Event {
    Arrived(i32),
    DoorOpened,
    DoorClosed,
    LobbyCall(i32, Direction),
    CarCall(i32)
}

/// A direction of travel.
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

/// The car has arrived on the given floor.
fn car_arrived(floor: i32) -> Event {
    Event::Arrived(floor)
}

/// The car doors have opened.
fn car_door_opened() -> Event {
    Event::DoorOpened
}

/// The car doors have closed.
fn car_door_closed() -> Event {
    Event::DoorClosed
}

/// A directional button was pressed in an elevator lobby on the given floor.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::LobbyCall(floor, dir)
}

/// A floor button was pressed in the elevator car.
fn car_floor_button_pressed(floor: i32) -> Event {
    Event::CarCall(floor)
}

fn main() {
    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!("The car has arrived on the ground floor: {:?}", car_arrived(0));
    println!("The car door opened: {:?}", car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));
}