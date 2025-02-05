use std::collections::HashMap;

#[derive(Debug)]
struct SensorData {
    id: usize,
    value: f64,
}

fn main() {
    let mut sensors: Vec<SensorData> = Vec::with_capacity(1_000_000);
    for id in 1..=1_000_000 {
        sensors.push(SensorData { id, value: rand::random::<f64>() });
    }

    // Analyze data
    let average_value: f64 = sensors.iter().map(|s| s.value).sum::<f64>() / sensors.len() as f64;
    println!("Average sensor value: {}", average_value);

    // HashMap for IDs and values
    let mut sensor_map: HashMap<usize, f64> = HashMap::new();
    for sensor in &sensors {
        sensor_map.insert(sensor.id, sensor.value);
    }

    // Searching for a specific sensor ID
    if let Some(value) = sensor_map.get(&500_000) {
        println!("Value for sensor 500000: {}", value);
    }

    // Performance comparisons would depend on actual benchmarks, but generally, Rust’s performance is comparable to C/C++.

    // Example for memory optimization using Box
    let boxed_sensors: Vec<Box<SensorData>> = sensors.into_iter().map(|s| Box::new(s)).collect();
}