
fn main() {
    let input = include_str!("./input.txt");
    let almanac = parse_input(input);

    println!(
        "Lowest location to plant: {}",
        find_lowest_location_to_plant(&almanac)
    );
}

#[derive(Debug, Eq, PartialEq)]
struct Range {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

struct Almanac {
    seeds_to_pant: Vec<i64>,
    seeds_to_soil_map: Vec<Range>,
    soil_to_fertilizer_map: Vec<Range>,
    fertilizer_to_water_map: Vec<Range>,
    water_to_light_map: Vec<Range>,
    light_to_temperature_map: Vec<Range>,
    temperature_to_humidity_map: Vec<Range>,
    humidity_to_location_map: Vec<Range>,
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
        seeds_to_pant: Vec::new(),
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
    result.seeds_to_pant = seeds;

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

    assert_eq!(result.seeds_to_pant, vec![79, 14, 55, 13]);
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
        }
    ];

    assert_eq!(convert(79, &ranges), 81);
}

fn find_lowest_location_to_plant(almanac: &Almanac) -> i64 {
    let mut lowest_location = i64::MAX;

    for seed in &almanac.seeds_to_pant {
        let soil = convert(*seed, &almanac.seeds_to_soil_map);
        let fertilizer = convert(soil, &almanac.soil_to_fertilizer_map);
        let water = convert(fertilizer, &almanac.fertilizer_to_water_map);
        let light = convert(water, &almanac.water_to_light_map);
        let temperature = convert(light, &almanac.light_to_temperature_map);
        let humidity = convert(temperature, &almanac.temperature_to_humidity_map);
        let location = convert(humidity, &almanac.humidity_to_location_map);

        println!("seed: {}, soil: {}, fertilizer: {}, water: {}, light: {}, temperature: {}, humidity: {}, location: {}", seed, soil, fertilizer, water, light, temperature, humidity, location);
        if location < lowest_location {
            lowest_location = location;
        }
    }

    lowest_location
}

#[test]
fn test_find_lowest_location_to_plant() {
    let input = include_str!("./example1.txt");
    let almanac = parse_input(input);

    assert_eq!(find_lowest_location_to_plant(&almanac), 35);
}