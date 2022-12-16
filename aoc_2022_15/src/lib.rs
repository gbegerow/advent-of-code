use std::ops::Range;

use regex::Regex;
// use std::collections::HashMap;
// use std::io::{stdout, Write};

// use crossterm::{
//     cursor,
//     style::{self, Stylize},
//     terminal, QueueableCommand, Result,
// };

// #[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coordinate { x: i64, y: i64 }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SensorArea {
    sensor: Coordinate,
    beacon: Coordinate,
    distance: i64,
}

fn distance(sx:i64, sy:i64, bx: i64, by:i64) -> i64 {
    // be carefull, coordinates might be negative, so just shift by minimum into positive
    // let minx = (if sx < bx {sx} else {bx}).abs() + 1;
    // let miny = (if sy < by {sy} else {by}).abs() + 1;
    
    // ((sx + minx) - (bx + minx)).abs() + ((sy + miny) - (by + miny)).abs()
    (sx - bx).abs() + (sy - by).abs()
}

impl SensorArea {
    fn new(sx:i64, sy:i64, bx: i64, by:i64) -> Self {
        
        Self { 
            sensor: Coordinate { x: sx, y: sy }, 
            beacon: Coordinate { x: bx, y: by },
            distance:  distance(sx, sy, bx, by),
        } }
}

// #[derive(Debug, PartialEq, Eq)]
// enum  Tile {
//     Sensor,
//     Beacon,
//     Known,
// }

fn parse(input: &str) -> Vec<SensorArea> {
    // no need for complex parser
    // Sensor at x=2317632, y=2942537: closest beacon is at x=2342391, y=2905242
    let rx = Regex::new(r"Sensor at x=(?P<sx>\d+), y=(?P<sy>\d+): closest beacon is at x=(?P<bx>\d+), y=(?P<by>\d+)").unwrap();
    let mut data = Vec::with_capacity(100);
    for caps in rx.captures_iter(input) {
        data.push(SensorArea::new(
            caps["sx"].parse().unwrap(),
            caps["sy"].parse().unwrap(),
            caps["bx"].parse().unwrap(),
            caps["by"].parse().unwrap()
        ));
    }
    data
}

// fn get_bounds(data: &Vec<SensorArea>) -> (Coordinate, Coordinate) {
//     let (mut minx, mut maxx, mut miny, mut maxy) = (i64::MAX, i64::MIN,i64::MAX, i64::MIN);
//     for d in data{
//         // a sensor can look distance away so min is x - distance
//         if d.sensor.x - d.distance < minx { minx = d.sensor.x - d.distance;} 
//         if d.sensor.x + d.distance > maxx { maxx = d.sensor.x + d.distance;} 
//         if d.sensor.y - d.distance < miny { miny = d.sensor.y - d.distance;} 
//         if d.sensor.y + d.distance > maxy { maxy = d.sensor.y + d.distance;} 

//         // beacon must be in distance so skip this test
//         // if d.beacon.x < minx { minx = d.beacon.x;} 
//         // if d.beacon.x > maxx { maxx = d.beacon.x;} 
//         // if d.beacon.y < miny { miny = d.beacon.y;} 
//         // if d.beacon.y > maxy { maxy = d.beacon.y;}     
//     }
//     (Coordinate {x:minx, y:miny}, Coordinate {x:maxx,y:maxy})
// }


fn visible(d: &SensorArea, y: i64) -> Range<i64> {
    // what part of the row is visible from sensor?
    let miny = (if d.sensor.y < y {d.sensor.y} else {y}).abs() + 1;
    let y_dist = ((d.sensor.y + miny) - (y + miny)).abs(); // shift y's to positive
    if y_dist > d.distance {return 1..-1} // nothing visibile in this row

    // let projection = (d.distance - y_dist) * 2 + 1; 
    // projection

    let projection_distance = d.distance - y_dist;
    d.sensor.x - projection_distance .. d.sensor.x + projection_distance + 1
}

pub fn aoc_2022_15_a(input: &str, sensor_row: i64) -> usize {
    let data = parse(input);
    // let bounds = get_bounds(&data);
    // println!("{:?}", bounds);

    // store minimal, just use magic erm math
    let mut beacons = Vec::with_capacity(data.len());
    let mut sensors = Vec::with_capacity(data.len());
    let mut ranges = Vec::with_capacity(data.len());
    for d in data  {
        ranges.push( visible(&d, sensor_row)); // collect ranges to deduplicate them

        // beacons and sensor on line do not count but only once
        if d.beacon.y == sensor_row && !beacons.contains(&d.beacon) { beacons.push(d.beacon.clone()); }
        if d.sensor.y == sensor_row && !sensors.contains(&d.sensor) { sensors.push(d.sensor.clone()); }
    }
    
    // merge ranges to not count cells twice
    ranges.sort_unstable_by_key(|r| r.start); // sort so we unidirectional
    println!("{:?}", ranges);
    let mut merged_ranges = Vec::with_capacity(ranges.len());
    let mut current = 0..-1; // empty range    
    for r in ranges {
        if r.is_empty() {continue;}
        if current.is_empty() {
            current = r.start .. r.end;
        } else if r.start <= current.end + 1  && r.end > current.end { // overlap or adjacent, merge
            current = current.start .. r.end
        } else if r.start > current.end +1 {
            // new run, remember the old
            merged_ranges.push(current.clone());
            current = r.start .. r.end;
        } 
    }
    merged_ranges.push(current); // last range is not in list

    println!("{:?}", merged_ranges);

    let known = 
        merged_ranges.iter().map(|r| r.len()).sum::<usize>()
            - sensors.len()
            - beacons.len();

    println!("known {} beacons {} sensors {}", known, beacons.len(), sensors.len());

    return known
}

pub fn aoc_2022_15_b(_input: &str) -> usize {
    0
}



#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{SensorArea, distance, visible};

    #[test]
    fn aoc_2022_15_a_example() {
        assert_eq!(super::aoc_2022_15_a(TEST_INPUT, 10), 26);
    }

    #[test]
    fn aoc_2022_15_a() {
       assert_eq!(super::aoc_2022_15_a(include_str!("input.txt"), 2000000), 0); // 4626838 to low??
    }
    
    #[test]
    fn aoc_2022_15_b_example() {
        assert_eq!(super::aoc_2022_15_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2022_15_b() {
        assert_eq!(super::aoc_2022_15_b(include_str!("input.txt")), 0);
    }

    // ------- Copy cat. Port of u/Metarineo python solution just to get the number. But still got the same number????
    #[test]
    fn copy_cat(){
/*
        import re
        xPos = set()
        yLine = 2000000
        with open("20221215.txt") as fp: 
            lines = fp.read().splitlines()  
            for line in lines[:]:
                row = re.sub("[^0-9=-]","", line)[1:].split('=')
                sx,sy,bx,by = row
                sx = int(sx); sy = int(sy); bx = int(bx); by = int(by)
                myBDist = abs(bx-sx)+abs(by-sy)
                myYDist = abs(yLine-sy)
                if myYDist <= myBDist:
                    for i in range (sx-(myBDist-myYDist),sx+(myBDist-myYDist)):
                        xPos.add(i)
        print("Def not beacon Pos: ", len(xPos))
        */
        const SENSOR_ROW:i64 = 2000000;
        let data = super::parse(include_str!("input.txt"));
        let mut  x_pos = HashSet::with_capacity(2048);
        for d in data {
            let beacon_dist = (d.beacon.x - d.sensor.x).abs() + (d.beacon.y - d.sensor.y).abs();    
            let y_dist = (SENSOR_ROW - d.sensor.y).abs();

            if y_dist < beacon_dist {
                let offset = beacon_dist-y_dist;
                for i in d.sensor.x - offset.. d.sensor.x + offset+1 {
                    x_pos.insert(i);
                }
            }
        }
        println!("Def not beacon pos: {}", x_pos.len())
}

    // ---------------- Unit Tests -------------
    #[test]
    fn test_visible(){
        assert_eq!(visible(&SensorArea::new(8,7, 2,10), 10), 2..16);
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            super::parse("Sensor at x=2317632, y=2942537: closest beacon is at x=2342391, y=2905242"),
            vec!(SensorArea::new(2317632, 2942537,2342391,2905242))
        );
    }

    #[test]
    fn test_distance_negative_coordinates() {
        assert_eq!(SensorArea::new(10, 0,-2,10).distance, 22);
    }

    
    #[test]
    fn test_beacon_in_distance() {
        let d = SensorArea::new(8,7, 2,10);
        assert_eq!( distance(d.sensor.x, d.sensor.y, d.beacon.x, d.beacon.y), d.distance , "beacon not in distance")
    }

    #[test]
    fn test_all_beacons_in_distance() {
        let data = super::parse(TEST_INPUT);
        assert!(data.iter().all(|d| distance(d.sensor.x, d.sensor.y, d.beacon.x, d.beacon.y) <= d.distance ), "Not all beacons are in distance")
    }

    // #[test]
    // fn bounds_should_be() {
    //     let data = super::parse(TEST_INPUT);
    //     assert_eq!(super::get_bounds(&data), 
    //         // ( Coordinate { x:-2 ,y:0}, Coordinate { x:25 ,y:22} )
    //         ( Coordinate { x:-8 ,y:-4}, Coordinate { x:28 ,y:28} )
    //     )
    // }

    const TEST_INPUT: &str = "
    Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    Sensor at x=9, y=16: closest beacon is at x=10, y=16
    Sensor at x=13, y=2: closest beacon is at x=15, y=3
    Sensor at x=12, y=14: closest beacon is at x=10, y=16
    Sensor at x=10, y=20: closest beacon is at x=10, y=16
    Sensor at x=14, y=17: closest beacon is at x=10, y=16
    Sensor at x=8, y=7: closest beacon is at x=2, y=10
    Sensor at x=2, y=0: closest beacon is at x=2, y=10
    Sensor at x=0, y=11: closest beacon is at x=2, y=10
    Sensor at x=20, y=14: closest beacon is at x=25, y=17
    Sensor at x=17, y=20: closest beacon is at x=21, y=22
    Sensor at x=16, y=7: closest beacon is at x=15, y=3
    Sensor at x=14, y=3: closest beacon is at x=15, y=3
    Sensor at x=20, y=1: closest beacon is at x=15, y=3
    ";
}


// old try gives out of memory if all are stored
/* V2 incorrect

fn insert_row(data : &SensorArea, y: i64, map: &mut HashMap<Coordinate, Tile>) {
    let sensor = &data.sensor;
    let beacon = &data.beacon;
    let dist = data.distance;

    if sensor.y - dist > y || sensor.y + dist < y {return} // to far away to influence this row

    for x in sensor.x - dist -1 .. sensor.x + dist +1 {
        
        if distance(sensor.x, sensor.y, x, y) > dist {continue;} // out of reach

        let c = Coordinate{x,y};
        match map.get(&c) {
            Some(&Tile::Sensor) | Some(&Tile::Beacon) => (), // occupied, do nothing 
            _ => {
                let mut tile = Tile::Known; 
                if c.x == sensor.x && c.y == sensor.y { tile= Tile::Sensor;}
                if c.x == beacon.x && c.y == beacon.y { tile = Tile::Beacon; }

                map.insert(c, tile);
                () // arms must have same type
            },
        };
    }
}

pub fn aoc_2022_15_a(input: &str, sensor_row: i64) -> usize {
    let data = parse(input);
    let bounds = get_bounds(&data);
    println!("{:?}", bounds);

    let mut  map = HashMap::with_capacity(bounds.1.x as usize);
    for d in data {
        insert_row(&d, sensor_row, &mut map);
    }
    // draw_cave(&map, &bounds).unwrap_or_default();

    let mut row: Vec<_> = map.iter()
        .filter(|(c,_)| c.y == sensor_row && map.get(c) == Some(&Tile::Known))
        .map(|(c, t)| (c.x, t))
        .collect();
    row.sort_unstable_by_key(|(x, _)| x.clone() );

    // println!("{} from {:?}", row.len(), row);

    row.len()
}

*/
/* V1 get all in hash gives out of memory

pub fn aoc_2022_15_a(input: &str, sensor_row: i64) -> usize {
    let data = parse(input);
    let bounds = get_bounds(&data);
    
    let mut map = HashMap::with_capacity(bounds.1.x as usize * bounds.1.y as usize);
    for d in data { insert(&d, &mut map);}

    // draw_cave(&map, &bounds).unwrap_or_default();

    let mut row: Vec<_> = map.iter()
        .filter(|(c,_)| c.y == sensor_row && map.get(c) == Some(&Tile::Known))
        .map(|(c, t)| (c.x, t))
        .collect();
    row.sort_unstable_by_key(|(x, _)| x.clone() );

    println!("{} from {:?}", row.len(), row);

    row.len()
}

fn insert(data : &SensorArea, map: &mut HashMap<Coordinate, Tile>) {
    let sensor = &data.sensor;
    let beacon = &&data.beacon;
    let dist = data.distance;
    for y in sensor.y - dist-1 .. sensor.y + dist +1 {
        for x in sensor.x - dist -1 .. sensor.x + dist +1 {
            
            if distance(sensor.x, sensor.y, x, y) > dist {continue;} // out of reach

            let c = Coordinate{x,y};
            match map.get(&c) {
                Some(&Tile::Sensor) | Some(&Tile::Beacon) => (), // well known, do nothing 
                _ => {
                    let mut tile = Tile::Known; 
                    if c.x == sensor.x && c.y == sensor.y { tile= Tile::Sensor;}
                    if c.x == beacon.x && c.y == beacon.y { tile = Tile::Beacon; }

                    if c.x == 2 && c.y == 10 {println!("(2,10) => {:?}", tile);}
                     map.insert(c, tile);
                    () // arms must have same type
                },
            };
        }
    }
    debug_assert_eq!(map.get(sensor), Some(&Tile::Sensor), "Sensor not in map {:?}", data);
    debug_assert_eq!(map.get(beacon), Some(&Tile::Beacon), "Beacon not in map {:?}", data);

}

#[allow(dead_code)]
fn draw_cave(map: &HashMap<Coordinate, Tile>, bounds: &(Coordinate,Coordinate)) -> Result<()> {
    let mut stdout = stdout();

    stdout.queue(terminal::Clear(terminal::ClearType::All))?;
    // stdout.queue(cursor::MoveToNextLine(1))?;
    // bounds upper left, lower right
    let window = terminal::size()?;
    let (minx, w) = (bounds.0.x, bounds.1.x - bounds.0.x);
    let (miny, h) = (bounds.0.y, bounds.1.y - bounds.0.y);
    let row = |c:&Coordinate| ((c.x  - minx) * w / window.0 as i64) as u16;
    let col = |c:&Coordinate| ((c.y  - miny) * h / window.1 as i64) as u16;

    for (c, t) in map {
        stdout.queue(cursor::MoveTo(col(c), row(c)))?; 
        match t {
            // Tile::Unknown => stdout.queue(style::PrintStyledContent(".".grey()))?,
            Tile::Sensor => stdout.queue(style::PrintStyledContent("S".white()))?,
            Tile::Beacon => stdout.queue(style::PrintStyledContent("B".yellow()))?,
            Tile::Known => stdout.queue(style::PrintStyledContent(".".grey()))?,
        };
    }

    stdout.queue(cursor::MoveToRow(row(&bounds.1) + 1))?;
    stdout.flush()?;
    Ok(())
}
*/