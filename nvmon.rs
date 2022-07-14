
use std::process::Command;
use std::time::Duration;
use std::thread::sleep;
use std::str;


fn main() {

  let o = 12;
  let mut nvs: Vec<String>;
  let mut utilization: usize;
  let mut gpu = vec![0; 60];
  
  print!("{}[2J", 27 as char);

  while true {

    nvs = nvidia_string();
    utilization = nvs[o+15].parse::<usize>().unwrap();
    shift_insert_vec(&mut gpu, utilization);

    print!("\x1B[1;1H");
    println!("{} {} {} {}", nvs[o+0], nvs[o+2], nvs[o+3], nvs[o+4]);
    println!("{}°C {}W {}MHz {}MB/{}MB       ", nvs[o+6], nvs[o+7], nvs[o+9], nvs[o+11], nvs[o+13]);
    println!("");
    println!("\tutilization: {}%", nvs[o+15]);
    simple_plot(&gpu, 10, 100);

    sleep(Duration::from_millis(500));
  }

}

fn shift_insert_vec(v: &mut Vec<usize>, new_i: usize) {
  let len = v.len();
  for i in 0..v.len()-1 {
    v[i] = v[i+1];
  }
  v[len-1] = new_i
}

fn nvidia_string() -> Vec<String> {
  let cmd = "nvidia-smi".to_string();
  let output = Command::new(cmd).args(["--query-gpu=name,temperature.gpu,power.draw,clocks.sm,memory.used,memory.total,utilization.gpu", "--format=csv"]).output().expect("failed to execute process");
  let name = str::from_utf8(&output.stdout).unwrap();
  
  let name: Vec<String> = name.split(&[' ', '\n'][..]).map(|s| s.to_string()).collect();
  return name
}


fn simple_plot(vec: &Vec<usize>, hoehe: usize, scala: usize) {
  for x in 0..hoehe+1 {
    directplot(&vec, scala/hoehe, hoehe-x);
  }
}

fn directplot(vec: &Vec<usize>, i: usize, step: usize) {
  const EMPTY: String = String::new();
  let mut arr = vec![EMPTY; vec.len()];

  for x in 1..vec.len() {
    arr[x] = " ".to_string();

    //filler
    if vec[x] >= i*(step+1) && vec[x-1] <= i*step {
      arr[x] = "│".to_string();
    }
    if vec[x] < i*(step+1) && vec[x-1] >= i*step {
      arr[x] = "│".to_string();
    }

    //ende
    if vec[x] >= i*step && vec[x] < i*(step+1) {
      if vec[x] > vec[x-1] {
        arr[x] = "┌".to_string();
      }
      if vec[x] < vec[x-1] {
        arr[x] = "└".to_string();
      }
      if vec[x]/i == vec[x-1]/i {
        arr[x] = "─".to_string();
      }
    } 

    //anfang
    if vec[x-1]/i >= step && vec[x-1]/i <= step {
      if vec[x] > vec[x-1] {
        arr[x] = "┘".to_string();
      }
      if vec[x] < vec[x-1] {
        arr[x] = "┐".to_string();
      }
      if vec[x]/i == vec[x-1]/i {
        arr[x] = "─".to_string();
      }
    }

  }

  let joined = arr.join("");
  println!("{}\t -{}-", i*step, joined);
}
