use std::fs;
use std::cmp;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone)]
struct Source {
    start: i64,
    end: i64,
}

impl Source {

    fn overlaps(&self, map: &SrcDstMap) -> bool {
            !( (self.end < map.src_start) || (self.start >= map.src_start + map.range) )
    }

    fn overlaps_any(&self, maps: &SrcDstMaps) -> bool {
        maps.maps.iter().any(|m| self.overlaps(m))
    }

}

#[derive(Debug, Clone)]
struct Sources {
    sources: Vec<Source>
}

impl Sources {
    fn prune(mut self) -> Self {
        self.sources.sort_by_key(|s| s.start);
        // println!("pruning");
        // println!("{:?}", self);



        let mut non_pruned: VecDeque<Source> = self.sources.into();
        let mut pruned: Vec<Source> = Vec::new();

        while let Some(seed) = non_pruned.pop_front() {
            let mut new_seed = seed;

            while let Some(seed) = non_pruned.pop_front() {
                if seed.start > new_seed.end + 1{ //disjoint
                    non_pruned.push_front(seed);
                    break;
                }
                new_seed.end = cmp::max(new_seed.end, seed.end);
            }
            pruned.push(new_seed);
        }
        self.sources = pruned;
        // println!("{:?}", self);

        self
    }

    fn new() -> Sources {
        Sources { sources: Vec::new() }
    }
    
    fn is_empty(&self) -> bool {
        self.sources.is_empty()
    }

    fn min(&self) -> i64 {
        self.sources.iter().map(|s| s.start).min().unwrap_or(0)
    }
}

#[derive(Debug, Copy, Clone)]
struct SrcDstMap {
    src_start: i64,
    range: i64,
    dst_offset: i64,
}

impl SrcDstMap {
    fn map(&self, src: Source) -> (Source, Vec<Source>) {
        // println!("\tmapping {:?} with {:?}", src, self);
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

#[derive(Debug, Clone)]
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
                    break;
                }
            }
            if !consumed {
                mapped.push(src);
            }
        }

        Sources{sources: mapped}
    }
}

fn parse_to_seeds(l: &str) -> Sources {
    match l.split_once(":") {
        Some(("seeds", seeds)) => {
            let v: Vec<Source> = seeds
                .split(" ")
                .filter_map(|s| s.trim().parse::<i64>().ok())
                .map(|n| Source{start: n, end: n})
                .collect();
            Sources{sources: v}
        }
        _ => Sources{sources: Vec::new()}
    }
}

fn main() {
    let contents = fs::read_to_string("input_files/day_5/part1.txt").expect("No such file found!");

    let mut lines = contents.lines();

    let mut seeds: Sources = Sources{sources: Vec::new()};
    // let mut seeds: Sources = Sources{sources: vec![Source{start: 79, end: 79},Source{start: 14, end: 14},Source{start: 55, end: 55},Source{start: 13, end: 13} ]};

    // ALMANACH:
    let mut almanach: Vec<SrcDstMaps> = Vec::new();

    while let Some(line) = lines.next() {
        if seeds.is_empty() {
            seeds = parse_to_seeds(line);
            // continue;
        } else {
            match line.find("map") {
                Some(_) => {
                    let mut maps: Vec<SrcDstMap> = Vec::new();
                    while let Some(line) = lines.next() {
                        let v: Vec<i64> = line
                            .split(" ")
                            .filter_map(|s| s.trim().parse::<i64>().ok())
                            .collect();
                        if v.len() == 3 {
                            maps.push(SrcDstMap{src_start: v[1], dst_offset: v[0] - v[1], range: v[2]});
                        } else {
                            almanach.push(SrcDstMaps{maps});
                            break;
                        }
                    }
                },
                _ => continue,
            }
        }
    }
    
    let number: i64 = almanach.iter().fold(seeds, |src, m| m.map(src).prune()).min();
    // let number: i64 = almanach.iter().fold(seeds, |src, m| m.map(src)).min();
    // almanach.iter().for_each(|m| seeds = m.map(seeds.clone()).prune());

    // let number: i64 = seeds.min();
    
    // let some = almanach[0].map(seeds.clone());;
    //
    // println!("The seed is: {:?}", seeds);
    // println!("The seed_to_soil is: {:?}", almanach[0]);
    // println!("The soil is: {:?}", some);
    println!("The number is: {number}");
    println!("");
    println!("{:?}", almanach.len());
}
