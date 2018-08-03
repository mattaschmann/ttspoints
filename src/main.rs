use std::env::args;

fn main() {
  let args: Vec<String> = args().collect();

  if args.len() != 3 {
    print!("You must pass in two arguments, hours and minutes");
    return;
  }

  let hours = match args[1].parse::<f64>() {
    Ok(h) => h,
    Err(_) => {
      print!("hours argument was of wrong type!");
      return;
    }
  };
  let minutes = match args[2].parse::<f64>() {
    Ok(m) => m,
    Err(_) => {
      print!("minutes argument was of wrong type!");
      return;
    }
  };

  print!("{}", ttspoints(&hours, &minutes).to_string())
}

/**
 * Convert an hour and minute into story points
 */
fn ttspoints(hours: &f64, minutes: &f64) -> f64 {
  (hours + minutes / 60.0) / 8.0
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
