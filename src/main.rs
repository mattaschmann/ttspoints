use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let hours = &args[1].parse::<f64>().unwrap();
  let minutes = &args[2].parse::<f64>().unwrap();

  print!("{}", ttspoints(hours, minutes).to_string())
}

/**
 * Convert an hour and minute into story points
 */
fn ttspoints(hours: &f64, minutes: &f64) -> f64 {
  let minutes_fraction = minutes / 60.0;
  let time = hours + minutes_fraction;

  time / 8.0
}

#[test]
fn time_to_storypoints_test() {
  assert_eq!(ttspoints(&8.0, &0.0), 1.0);
  assert_eq!(ttspoints(&4.0, &0.0), 0.5);
  assert_eq!(ttspoints(&12.0, &42.0), 1.5875);
}

#[test]
fn main_parse_args() {
  let hour_str = "8";
  let min_str = "0";
  assert_eq!(ttspoints(&hour_str.parse::<f64>().unwrap(), &min_str.parse::<f64>().unwrap()), 1.0);
}
