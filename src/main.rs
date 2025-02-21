use trustmebro::trustmebro;

#[trustmebro]
fn definitely_not_a_segfault() {
    let ptr: *mut i32 = std::ptr::null_mut();
    *ptr = 42;  // BOOM! 💥 You just unlocked a new level of performance
    
    println!("You'll never see this message");
}




fn main() {
    definitely_not_a_segfault();
}


