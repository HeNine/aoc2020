use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").unwrap();

    let input_buffer = BufReader::new(&file);
    let mut lines = input_buffer.lines();

    let time: i64 = lines.next().unwrap().unwrap().parse().unwrap();
    let (mut buses, mut counts, _): (Vec<i64>, Vec<i64>, i64) = lines.next().unwrap().unwrap()
        .split(',')
        .map(|num| match num.parse() {
            Ok(num) => Some(num),
            Err(_e) => None
        })
        .fold((Vec::new(), Vec::new(), 0),
              |(mut ids, mut counts, count), id| match id {
                  Some(id) => {
                      ids.push(id);
                      counts.push(count);
                      (ids, counts, count + 1)
                  }
                  None => (ids, counts, count + 1)
              });

    println!("{}", time);
    println!("{:?}", buses);
    println!("{:?}", counts);

    let mut i = 0;
    loop {
        if i == buses.len() - 1 { break; }
        if buses[i + 1] > buses[i] {
            buses.swap(i, i + 1);
            counts.swap(i, i + 1);
            if i > 0 { i -= 1 }
        } else {
            i += 1
        }
    }

    println!("{:?}", buses);
    println!("{:?}", counts);

    let mut xi = (buses[0] - counts[0]).rem_euclid(buses[0]);

    let mut prod_n = buses[0];

    for i in 1..buses.len() {
        let ai = (buses[i] - counts[i]).rem_euclid( buses[i]) ;
        let ni = buses[i];


        loop {
            if xi % ni == ai { break; }
            xi += prod_n;
        }

        prod_n *= ni;
    }

    println!("{}", xi);

    // let mut min_time = *buses.iter().max().unwrap();
    // let mut min_bus= 0;
    // for bus_id in buses.iter()  {
    //
    //     let wait_time = bus_id-time%bus_id;
    //
    //     if wait_time < min_time {
    //         min_time = wait_time;
    //         min_bus = *bus_id;
    //     }
    // }
    //
    // println!("{}", min_bus*min_time);
}
