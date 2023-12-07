fn main() {
    let input = include_str!("./input.txt");
    let almanac = parse_input(input);

    println!(
        "Lowest location to plant: {}",
        almanac.find_lowest_location_to_plant()
    );

    let almanac2 = Almanac2::from(almanac);
    println!(
        "Lowest location to plant using ranges: {}",
        almanac2.find_lowest_location_to_plant()
    );
}

#[derive(Debug, Eq, PartialEq)]
struct Range {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

struct Almanac {
    seeds_to_plant: Vec<i64>,
    seeds_to_soil_map: Vec<Range>,
    soil_to_fertilizer_map: Vec<Range>,
    fertilizer_to_water_map: Vec<Range>,
    water_to_light_map: Vec<Range>,
    light_to_temperature_map: Vec<Range>,
    temperature_to_humidity_map: Vec<Range>,
    humidity_to_location_map: Vec<Range>,
}

#[derive(Debug, Eq, PartialEq)]
struct Almanac2 {
    seed_ranges_to_plant: Vec<(i64, i64)>,
    seeds_to_soil_map: Vec<Range>,
    soil_to_fertilizer_map: Vec<Range>,
    fertilizer_to_water_map: Vec<Range>,
    water_to_light_map: Vec<Range>,
    light_to_temperature_map: Vec<Range>,
    temperature_to_humidity_map: Vec<Range>,
    humidity_to_location_map: Vec<Range>,
}

impl From<Almanac> for Almanac2 {
    fn from(value: Almanac) -> Self {
        let mut result = Self {
            seed_ranges_to_plant: Vec::new(),
            seeds_to_soil_map: value.seeds_to_soil_map,
            soil_to_fertilizer_map: value.soil_to_fertilizer_map,
            fertilizer_to_water_map: value.fertilizer_to_water_map,
            water_to_light_map: value.water_to_light_map,
            light_to_temperature_map: value.light_to_temperature_map,
            temperature_to_humidity_map: value.temperature_to_humidity_map,
            humidity_to_location_map: value.humidity_to_location_map,
        };

        value.seeds_to_plant.chunks(2).for_each(|chunk| {
            result.seed_ranges_to_plant.push((chunk[0], chunk[1]));
        });

        result
    }
}

#[test]
fn test_almanac2_from_almanac() {
    let input = include_str!("./example1.txt");
    let almanac = parse_input(input);
    let almanac2 = Almanac2::from(almanac);

    assert_eq!(almanac2.seed_ranges_to_plant, vec![(79, 14), (55, 13)]);
}


fn parse_map_block(lines: &mut std::str::Lines, ranges: &mut Vec<Range>) {
    lines.next(); // label line
    let mut line = lines.next().unwrap();
    while !line.is_empty() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let destination_range_start = parts[0].parse::<i64>().unwrap();
        let source_range_start = parts[1].parse::<i64>().unwrap();
        let range_length = parts[2].parse::<i64>().unwrap();

        ranges.push(Range {
            destination_range_start,
            source_range_start,
            range_length,
        });

        line = lines.next().unwrap_or("");
    }
}

fn parse_input(input: &str) -> Almanac {
    let mut result = Almanac {
        seeds_to_plant: Vec::new(),
        seeds_to_soil_map: Vec::new(),
        soil_to_fertilizer_map: Vec::new(),
        fertilizer_to_water_map: Vec::new(),
        water_to_light_map: Vec::new(),
        light_to_temperature_map: Vec::new(),
        temperature_to_humidity_map: Vec::new(),
        humidity_to_location_map: Vec::new(),
    };

    let mut lines = input.lines();

    let seeds_line = lines.next().unwrap();
    let seeds = seeds_line.replace("seeds: ", "").split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    result.seeds_to_plant = seeds;

    // blank line
    lines.next();

    // Parse ALL the maps!
    parse_map_block(&mut lines, &mut result.seeds_to_soil_map);
    parse_map_block(&mut lines, &mut result.soil_to_fertilizer_map);
    parse_map_block(&mut lines, &mut result.fertilizer_to_water_map);
    parse_map_block(&mut lines, &mut result.water_to_light_map);
    parse_map_block(&mut lines, &mut result.light_to_temperature_map);
    parse_map_block(&mut lines, &mut result.temperature_to_humidity_map);
    parse_map_block(&mut lines, &mut result.humidity_to_location_map);


    result
}

#[test]
fn test_parse_input() {
    let input = include_str!("./example1.txt");
    let result = parse_input(input);

    assert_eq!(result.seeds_to_plant, vec![79, 14, 55, 13]);
    assert_eq!(result.seeds_to_soil_map.len(), 2);
    assert_eq!(result.seeds_to_soil_map[0], Range {
        destination_range_start: 50,
        source_range_start: 98,
        range_length: 2,
    });
    assert_eq!(result.humidity_to_location_map.len(), 2);
    assert_eq!(result.humidity_to_location_map[0], Range {
        destination_range_start: 60,
        source_range_start: 56,
        range_length: 37,
    });
}

fn convert(source: i64, ranges: &Vec<Range>) -> i64 {
    let mut result = source;

    for range in ranges {
        if source >= range.source_range_start && source < range.source_range_start + range.range_length {
            result = range.destination_range_start + (source - range.source_range_start);
            break;
        }
    }

    result
}

#[test]
fn test_convert() {
    let ranges = vec![
        Range {
            source_range_start: 50,
            destination_range_start: 98,
            range_length: 2,
        }
    ];

    assert_eq!(convert(50, &ranges), 98);
    assert_eq!(convert(51, &ranges), 99);
    assert_eq!(convert(52, &ranges), 52);
}

#[test]
fn test_convert2() {
    let ranges = vec![
        Range {
            source_range_start: 98,
            destination_range_start: 50,
            range_length: 2,
        },
        Range {
            source_range_start: 50,
            destination_range_start: 52,
            range_length: 48,
        },
    ];

    assert_eq!(convert(79, &ranges), 81);
}

impl Almanac {
    fn find_lowest_location_to_plant(self: &Almanac) -> i64 {
        let mut lowest_location = i64::MAX;

        for seed in &self.seeds_to_plant {
            let soil = convert(*seed, &self.seeds_to_soil_map);
            let fertilizer = convert(soil, &self.soil_to_fertilizer_map);
            let water = convert(fertilizer, &self.fertilizer_to_water_map);
            let light = convert(water, &self.water_to_light_map);
            let temperature = convert(light, &self.light_to_temperature_map);
            let humidity = convert(temperature, &self.temperature_to_humidity_map);
            let location = convert(humidity, &self.humidity_to_location_map);

            if location < lowest_location {
                lowest_location = location;
            }
        }

        lowest_location
    }
}


#[test]
fn test_almanac_find_lowest_location_to_plant() {
    let input = include_str!("./example1.txt");
    let almanac = parse_input(input);

    assert_eq!(almanac.find_lowest_location_to_plant(), 35);
}

impl Almanac2 {
    fn find_lowest_location_to_plant(self: &Almanac2) -> i64 {
        let mut lowest_location = i64::MAX;

        for seed_range in &self.seed_ranges_to_plant {
            for seed in seed_range.0..(seed_range.0 + seed_range.1) {
                let soil = convert(seed, &self.seeds_to_soil_map);
                let fertilizer = convert(soil, &self.soil_to_fertilizer_map);
                let water = convert(fertilizer, &self.fertilizer_to_water_map);
                let light = convert(water, &self.water_to_light_map);
                let temperature = convert(light, &self.light_to_temperature_map);
                let humidity = convert(temperature, &self.temperature_to_humidity_map);
                let location = convert(humidity, &self.humidity_to_location_map);

                if location < lowest_location {
                    lowest_location = location;
                }
            }
        }

        lowest_location
    }
}

#[test]
fn test_almanac2_find_lowest_location_to_plant() {
    let input = include_str!("./example1.txt");
    let almanac = parse_input(input);
    let almanac2 = Almanac2::from(almanac);

    assert_eq!(almanac2.find_lowest_location_to_plant(), 46);
}
