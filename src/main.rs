use std::fs;
use std::cmp;

struct Seed {
    start: u64,
    end: u64,
}
struct Seeds {
    seeds: Vec<Seed>
}

impl Seeds {
    fn prune(self) -> Self {
        self.seeds.sort_by_key(|s| s.start);

        let mut new_seeds: Vec<Seed> = Vec::new();
        let mut it = self.seeds.iter();

        while let Some(seed) = it.next() {
            let mut new_seed = *seed;

            while let Some(seed) = it.next() {
                if seed.start > new_seed.end { //disjoint
                    break;
                }
                new_seed.end = cmp::max(new_seed.end, seed.end);
            }
            new_seeds.push(new_seed);


        }

    }
}




fn parse_to_seeds(l: &str) -> Vec<u64> {
    match l.split_once(":") {
        Some(("seeds", seeds)) => {
            let _v = seeds
                .split(" ")
                .filter_map(|s| s.trim().parse::<u64>().ok())
                .collect::<Vec<u64>>();
            let mut v: Vec<u64> = Vec::new();
            for chunk in _v.chunks(2) {
                if chunk.len() == 2 {
                    let from = chunk[0];
                    let to = chunk[0] + chunk[1];
                    let range: Vec<u64> = (from..to).collect();
                    v.extend(range);
                }
            }
            v
        }
        _ => Vec::new(),
    }
}

fn map_elems(e: u64, m: &Vec<(u64, u64, u64)>) -> u64 {
    for (dst, src, len) in m {
        if (e >= *src) && (e < *src + *len) {
            let dist = e - src;
            return dst + dist;
        }
    }
    return e;
}

fn main() {
    let contents = fs::read_to_string("input_files/day_5/part1.txt").expect("No such file found!");

    let mut lines = contents.lines();

    let mut seeds: Vec<u64> = Vec::new();

    // ALMANACH:

    let mut seed_to_soil: Vec<(u64, u64, u64)> = Vec::new();
    let mut soil_to_fertilizer: Vec<(u64, u64, u64)> = Vec::new();
    let mut fertilizer_to_water: Vec<(u64, u64, u64)> = Vec::new();
    let mut water_to_light: Vec<(u64, u64, u64)> = Vec::new();
    let mut light_to_temperature: Vec<(u64, u64, u64)> = Vec::new();
    let mut temperature_to_humidity: Vec<(u64, u64, u64)> = Vec::new();
    let mut humidity_to_location: Vec<(u64, u64, u64)> = Vec::new();

    while let Some(line) = lines.next() {
        if seeds.is_empty() {
            seeds = parse_to_seeds(line);
    println!("The seed is:");
        } else {
            match line {
                line if line.starts_with("seed-to-soil") => {
                    while let Some(line) = lines.next() {
                        let v: Vec<u64> = line
                            .split(" ")
                            .filter_map(|s| s.trim().parse::<u64>().ok())
                            .collect();
                        if v.len() == 3 {
                            seed_to_soil.push((v[0], v[1], v[2]));
                        } else {
                            break;
                        }
                    }
                }
                line if line.starts_with("soil-to-fertilizer") => {
                    while let Some(line) = lines.next() {
                        let v: Vec<u64> = line
                            .split(" ")
                            .filter_map(|s| s.trim().parse::<u64>().ok())
                            .collect();
                        if v.len() == 3 {
                            soil_to_fertilizer.push((v[0], v[1], v[2]));
                        } else {
                            break;
                        }
                    }
                }
                line if line.starts_with("fertilizer-to-water") => {
                    while let Some(line) = lines.next() {
                        let v: Vec<u64> = line
                            .split(" ")
                            .filter_map(|s| s.trim().parse::<u64>().ok())
                            .collect();
                        if v.len() == 3 {
                            fertilizer_to_water.push((v[0], v[1], v[2]));
                        } else {
                            break;
                        }
                    }
                }
                line if line.starts_with("water-to-light") => {
                    while let Some(line) = lines.next() {
                        let v: Vec<u64> = line
                            .split(" ")
                            .filter_map(|s| s.trim().parse::<u64>().ok())
                            .collect();
                        if v.len() == 3 {
                            water_to_light.push((v[0], v[1], v[2]));
                        } else {
                            break;
                        }
                    }
                }
                line if line.starts_with("light-to-temperature") => {
                    while let Some(line) = lines.next() {
                        let v: Vec<u64> = line
                            .split(" ")
                            .filter_map(|s| s.trim().parse::<u64>().ok())
                            .collect();
                        if v.len() == 3 {
                            light_to_temperature.push((v[0], v[1], v[2]));
                        } else {
                            break;
                        }
                    }
                }
                line if line.starts_with("temperature-to-humidity") => {
                    while let Some(line) = lines.next() {
                        let v: Vec<u64> = line
                            .split(" ")
                            .filter_map(|s| s.trim().parse::<u64>().ok())
                            .collect();
                        if v.len() == 3 {
                            temperature_to_humidity.push((v[0], v[1], v[2]));
                        } else {
                            break;
                        }
                    }
                }
                line if line.starts_with("humidity-to-location") => {
                    while let Some(line) = lines.next() {
                        let v: Vec<u64> = line
                            .split(" ")
                            .filter_map(|s| s.trim().parse::<u64>().ok())
                            .collect();
                        if v.len() == 3 {
                            humidity_to_location.push((v[0], v[1], v[2]));
                        } else {
                            break;
                        }
                    }
                }
                _ => continue,
            }
        }
    }

    let number: u64 = seeds
        .iter()
        .map(|e| map_elems(e.clone(), &seed_to_soil))
        .map(|e| map_elems(e, &soil_to_fertilizer))
        .map(|e| map_elems(e, &fertilizer_to_water))
        .map(|e| map_elems(e, &water_to_light))
        .map(|e| map_elems(e, &light_to_temperature))
        .map(|e| map_elems(e, &temperature_to_humidity))
        .map(|e| map_elems(e, &humidity_to_location))
        .min()
        .unwrap();

    println!("The seed_to_soil is: {:?}", map_elems(99, &seed_to_soil));
    println!("The number is: {number}");
}
