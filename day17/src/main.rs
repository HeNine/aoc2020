use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::ReactorCell::{Active, Inactive};

type Reactor = HashMap<(i32, i32, i32, i32), ReactorCell>;

#[derive(Copy, Clone, Debug)]
enum ReactorCell {
    Active(i32, i32, i32, i32),
    Inactive(i32, i32, i32, i32),
}

impl ReactorCell {
    fn new(state: char, x: i32, y: i32, z: i32, w: i32) -> Self {
        match state {
            '.' => Inactive(x, y, z, w),
            '#' => Active(x, y, z, w),
            _ => panic!()
        }
    }

    fn update(&self, reactor: &Reactor) -> Option<ReactorCell> {
        let (selfx, selfy, selfz, selfw) = match self {
            Active(x, y, z, w) | Inactive(x, y, z, w) => (*x, *y, *z, *w)
        };

        let mut count = 0;
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    for w in -1..=1 {
                        if x == 0 && y == 0 && z == 0 && w == 0 { continue; };
                        count += match reactor.get(&(selfx + x, selfy + y, selfz + z, selfw + w)) {
                            Some(Active(..)) => 1,
                            None | Some(Inactive(..)) => 0
                        }
                    }
                }
            }
        }

        match self {
            Inactive(x, y, z, w) => if count == 3 { Some(Active(*x, *y, *z, *w)) } else { None },
            Active(x, y, z, w) => if count == 2 || count == 3 { Some(Active(*x, *y, *z, *w)) } else { None }
        }
    }
}

fn main() {
    let file = File::open("input").unwrap();
    let file_buffer = BufReader::new(file);
    let mut lines = file_buffer.lines();


    let mut reactor = Reactor::new();

    let mut y = 0;
    while let Some(Ok(line)) = lines.next() {
        for (x, state) in line.chars().enumerate() {
            reactor.insert((x as i32, y, 0, 0), ReactorCell::new(state, x as i32, y, 0, 0));
        }
        y += 1;
    }

    print_z(&reactor, 0);

    for _ in 0..6 {
        let mut new_reactor = Reactor::new();
        let (min_x, min_y, min_z,min_w,
            max_x, max_y, max_z, max_w) = get_boundaries(&reactor);
        for x in min_x - 1..=max_x + 1 {
            for y in min_y - 1..=max_y + 1 {
                for z in min_z - 1..=max_z + 1 {
                    for w in min_w-1..=max_w+1 {
                        let new_cell = match reactor.get(&(x, y, z, w)) {
                            Some(x) => x.update(&reactor),
                            None => Inactive(x, y, z, w).update(&reactor)
                        };

                        match new_cell {
                            Some(Active(x, y, z,w)) => new_reactor.insert((x, y, z,w), new_cell.unwrap()),
                            Some(Inactive(..)) | None => None
                        };
                    }
                }
            }
        }

        reactor = new_reactor;
        // for z in min_z..max_z {
        //     println!("z={}", z);
        //     print_z(&reactor, z)
        // }
    }

    let count = reactor.values().filter(|cell| match cell {
        Active(..) => true,
        _ => false
    }).count();
    println!("{}", count);
}

fn print_z(reactor: &Reactor, z: i32) {
    let (min_x, min_y, _, _, max_x, max_y, _, _) = get_boundaries(reactor);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!("{}", match reactor.get(&(x, y, z, 0)) {
                Some(Active(..)) => '#',
                Some(Inactive(..)) | None => '.'
            })
        }
        println!();
    }
    println!();
}

fn get_boundaries(reactor: &Reactor) -> (i32, i32, i32,i32,
                                         i32, i32, i32,i32) {
    reactor.iter()
        .filter(|(_, state)| match state {
            Active(..) => true,
            Inactive(..) => false
        })
        .fold((i32::MAX, i32::MAX, i32::MAX, i32::MAX,
               i32::MIN, i32::MIN, i32::MIN,i32::MIN),
              |(minx, miny, minz, minw, maxx, maxy, maxz,maxw), ((x, y, z, w), _)| {
                  (min(minx, *x), min(miny, *y), min(minz, *z), min(minw, *w),
                   max(maxx, *x), max(maxy, *y), max(maxz, *z), max(maxw, *w))
              }
        )
}

