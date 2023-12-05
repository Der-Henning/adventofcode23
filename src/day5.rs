use super::utils::input;
use std::collections::HashMap;


struct Map {
    from_start: isize,
    to_start: isize,
    range: isize
}


struct Mapper {
    to: String,
    maps: Vec<Map>
}


fn parse_input() -> (Vec<isize>, HashMap<String, Mapper>) {
    let mut seeds: Vec<isize> = Vec::new();
    let mut mappers: HashMap<String, Mapper> = HashMap::new();
    input(&5).split("\n\n").for_each(|block: &str| {
        let b: Vec<&str> = block.split(":").collect();
        let name: &str = b[0].split(" ").collect::<Vec<&str>>()[0];
        let data: Vec<Vec<isize>> = b[1].trim().split("\n").map(|line: &str|
            line.trim()
            .split_whitespace().map(|x| 
                x.parse::<isize>().unwrap())
            .collect::<Vec<isize>>()
        ).collect::<Vec<Vec<isize>>>();
        if name == "seeds" {
            seeds = data[0].clone();
        }
        else {
            let a: Vec<&str> = name.split("-").collect::<Vec<&str>>();
            let mut maps = data.iter().map(|x: &Vec<isize>| Map {
                from_start: x[1],
                to_start: x[0],
                range: x[2]
            }).collect::<Vec<Map>>();
            maps.sort_by(|a: &Map, b: &Map| a.from_start.cmp(&b.from_start));

            maps.push(Map {
                from_start: maps[maps.len()-1].from_start + maps[maps.len()-1].range,
                to_start: maps[maps.len()-1].from_start + maps[maps.len()-1].range,
                range: isize::MAX - (maps[maps.len()-1].from_start + maps[maps.len()-1].range)
            });
            if maps[0].from_start > 0 {
                maps.insert(0, Map {
                    from_start: 0,
                    to_start: 0,
                    range: maps[0].from_start
                });
            }
            mappers.insert(a[0].to_string(), Mapper {
                to: a[2].to_string(),
                maps: maps
            });
        }
    });
    (seeds, mappers)
}


fn get_location(seed: &isize, mappers: &HashMap<String, Mapper>) -> isize {
    let mut mapper: String = "seed".to_string();
    let mut new_seed: isize = *seed;
    while mapper != "location".to_string() {
        let map: &Mapper = mappers.get(&mapper).unwrap();
        for m in map.maps.iter() {
            if new_seed >= m.from_start && new_seed <= m.from_start + m.range {
                new_seed = m.to_start + (new_seed - m.from_start);
                break;
            }
        }
        mapper = map.to.clone();
    }
    new_seed
}


pub fn task1() -> isize {
    let (seeds, mappers) = parse_input();
    seeds.iter().map(|seed: &isize| get_location(seed, &mappers)).min().unwrap()
}


pub fn task2() -> isize {
    let (seeds, mappers) = parse_input();
    seeds.chunks(2).map(|s: &[isize]| 
        get_min(&mappers, "seed".to_string(),s[0], s[1])
    ).min().unwrap()
}


fn get_min(mappers: &HashMap<String, Mapper>, level: String, from: isize, len: isize) -> isize {
    let mapper: &Mapper = mappers.get(&level).unwrap();
    let mut maps_iter = mapper.maps.iter();
    let mut map: &Map;
    let mut start: isize = from;
    let mut groups: Vec<(isize, isize)> = Vec::new();
    while start < from + len {
        map = maps_iter.next().unwrap();
        if start < map.from_start + map.range {
            let range: isize = if from + len < map.from_start + map.range {
                len - (start - from)
            } else {
                map.from_start + map.range - start
            };
            let diff: isize = map.to_start - map.from_start;
            groups.push((start + diff, range));
            start += range;
        }
    }
    if mapper.to == "location".to_string() {
        *groups.iter().map(|(from, _len)| from)
            .min().unwrap_or(&isize::MAX)
    } else {
        groups.iter().map(|(from, len)| {
            get_min(&mappers, mapper.to.clone(), *from, *len)
        }).min().unwrap_or(isize::MAX)
    }
}
