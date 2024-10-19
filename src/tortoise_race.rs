// use num::Float;

fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    let time_to_meet = -g as f32/(v1 as f32 - v2 as f32);
    let hours = time_to_meet.trunc();
    let minutes = time_to_meet.fract() * 60.0;
    let seconds = minutes.fract() * 60.0;//((time_to_meet - (hours as f32 + (minutes as f32/60.0))) * 60.0 * 60.0).ceil();
    let acc = vec![hours as i32,minutes as i32,seconds as i32];
    dbg!(&acc);
    if acc.iter().all(|num| num >= &0 ) {
        Some(acc)
    } else {
        None
    }
  }


fn main(){
    println!(
        "{:#?}", race(3, 91, 37)
    )
}