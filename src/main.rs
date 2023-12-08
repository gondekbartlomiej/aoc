use std::fs;
use std::cmp;
use std::collections::VecDeque;

#[derive(Copy, Clone)]
struct Source {
    start: i64,
    end: i64,
}

#[derive(Clone)]
struct Sources {
    sources: Vec<Source>
}

impl Source {

    fn overlaps(&self, map: &SrcDstMap) -> bool {
            !((self.end < map.src_start) || (self.start >= map.src_start + map.range))
    }

    fn overlaps_any(&self, maps: &SrcDstMaps) -> bool {
        maps.maps.iter().any(|m| self.overlaps(m))
    }

}

impl Sources {
    fn prune(mut self) -> Self {
        self.sources.sort_by_key(|s| s.start);

        let mut new_seeds: Vec<Source> = Vec::new();
        let mut it = self.sources.iter();

        while let Some(seed) = it.next() {
            let mut new_seed = *seed;

            while let Some(seed) = it.next() {
                if seed.start > new_seed.end + 1{ //disjoint
                    break;
                }
                new_seed.end = cmp::max(new_seed.end, seed.end);
            }
            new_seeds.push(new_seed);
        }
        self.sources = new_seeds;

        self
    }
}

#[derive(Copy, Clone)]
struct SrcDstMap {
    src_start: i64,
    range: i64,
    dst_offset: i64,
}

impl SrcDstMap {
    fn map(&self, src: Source) -> (Source, Vec<Source>) {
        // find left margin
        let mut non_mapped: Vec<Source> = Vec::new();


        let start = cmp::max(self.src_start, src.start) + self.dst_offset;
        let end = cmp::min(src.end, self.src_start + self.range -1) + self.dst_offset;
        let mapped = Source{start, end};

        let left_margin: bool = src.start < self.src_start;
        let right_margin: bool = src.end >= self.src_start + self.range;

        if left_margin {
            let end = self.src_start-1;
            non_mapped.push(Source{start: src.start, end});
        }
        if right_margin {
            let start = self.src_start + self.range;
            non_mapped.push(Source{start, end: src.end});
        }
        // find right margin
        // 
        return (mapped, non_mapped);
    }
}

#[derive(Clone)]
struct SrcDstMaps {
    maps: Vec<SrcDstMap>,
}

impl SrcDstMaps {
    fn map(&self, sources: Sources) -> Sources {
        let mut non_mapped = VecDeque::from(sources.sources);
        let mut mapped: Vec<Source> = Vec::new();

        while let Some(src) = non_mapped.pop_front() {
            let mut consumed = false;
            for m in self.maps.iter() {
                if src.overlaps(m) {
                    let (single_mapped, rest) = m.map(src);
                    mapped.push(single_mapped);
                    non_mapped.extend(rest);
                    consumed = true;
                }
            }
            if !consumed {
                mapped.push(src);
            }
        }

        Sources{sources: mapped}
    }
}

fn parse_to_seeds(l: &str) -> Vec<i64> {
    match l.split_once(":") {
        Some(("seeds", seeds)) => {
            let _v = seeds
                .split(" ")
                .filter_map(|s| s.trim().parse::<i64>().ok())
                .collect::<Vec<i64>>();
            let mut v: Vec<i64> = Vec::new();
            for chunk in _v.chunks(2) {
                if chunk.len() == 2 {
                    let from = chunk[0];
                    let to = chunk[0] + chunk[1];
                    let range: Vec<i64> = (from..to).collect();
                    v.extend(range);
                }
            }
            v
        }
        _ => Vec::new(),
    }
}

fn map_elems(e: i64, m: &Vec<(i64, i64, i64)>) -> i64 {
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

    let mut seeds: Vec<i64> = Vec::new();

    // ALMANACH:

    let mut seed_to_soil: Vec<(i64, i64, i64)> = Vec::new();
    let mut soil_to_fertilizer: Vec<(i64, i64, i64)> = Vec::new();
    let mut fertilizer_to_water: Vec<(i64, i64, i64)> = Vec::new();
    let mut water_to_light: Vec<(i64, i64, i64)> = Vec::new();
    let mut light_to_temperature: Vec<(i64, i64, i64)> = Vec::new();
    let mut temperature_to_humidity: Vec<(i64, i64, i64)> = Vec::new();
    let mut humidity_to_location: Vec<(i64, i64, i64)> = Vec::new();

    while let Some(line) = lines.next() {
        if seeds.is_empty() {
            seeds = parse_to_seeds(line);
    println!("The seed is:");
        } else {
            match line {
                line if line.starts_with("seed-to-soil") => {
                    while let Some(line) = lines.next() {
                        let v: Vec<i64> = line
                            .split(" ")
                            .filter_map(|s| s.trim().parse::<i64>().ok())
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
                        let v: Vec<i64> = line
                            .split(" ")
                            .filter_map(|s| s.trim().parse::<i64>().ok())
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
                        let v: Vec<i64> = line
                            .split(" ")
                            .filter_map(|s| s.trim().parse::<i64>().ok())
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
                        let v: Vec<i64> = line
                            .split(" ")
                            .filter_map(|s| s.trim().parse::<i64>().ok())
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
                        let v: Vec<i64> = line
                            .split(" ")
                            .filter_map(|s| s.trim().parse::<i64>().ok())
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
                        let v: Vec<i64> = line
                            .split(" ")
                            .filter_map(|s| s.trim().parse::<i64>().ok())
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
                        let v: Vec<i64> = line
                            .split(" ")
                            .filter_map(|s| s.trim().parse::<i64>().ok())
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

    let number: i64 = seeds
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
