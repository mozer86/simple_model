use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

use building_model::simulation_state::SimulationState;
use building_model::simulation_state_element::SimulationStateElement;

fn get_state_element(ini: usize, fin: usize) -> SimulationStateElement {
    let mut rng = rand::thread_rng();
    let n: usize = rng.gen_range(ini..fin);
    match n {
        // Individual ones
        0 => SimulationStateElement::Clothing(n as f64),

        // Operational ones
        1 => SimulationStateElement::FenestrationOpenFraction(n, n as f64),
        2 => SimulationStateElement::SpaceHeatingCoolingPowerConsumption(n, n as f64),
        3 => SimulationStateElement::SpaceLightingPowerConsumption(n, n as f64),

        // Physical ones
        4 => SimulationStateElement::SpaceDryBulbTemperature(n, n as f64),
        5 => SimulationStateElement::SurfaceNodeTemperature(n, n, n as f64),
        6 => SimulationStateElement::SpaceLoudness(n, n as f64),
        7 => SimulationStateElement::SpaceBrightness(n, n as f64),

        _ => panic!("There is no such StateElement"),
    }
}

fn get_some_flattened_state(n: usize) -> Vec<f64> {
    let mut state : Vec<f64> = Vec::with_capacity(n);
    let mut rng = rand::thread_rng();
    
    
    for _ in 0..n {
        let v : f64 = rng.gen();
        state.push(v);
    }
    state
}

fn get_some_simulation_state(n: usize) -> SimulationState {
    let mut state = SimulationState::new();

    // Add individual ones
    for _ in 0..n {
        state.push(get_state_element(0, 1));
    }

    // Add Operational ones
    for _ in 0..n {
        state.push(get_state_element(1, 4));
    }

    // Add physical ones
    for _ in 0..n {
        state.push(get_state_element(4, 8));
    }

    return state;
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let length = 1000;
    let state = get_some_simulation_state(length);
    // let state = get_some_flattened_state(length);

    c.bench_function("clone", |b| b.iter(|| black_box(state.clone())));

    let mut state_copy = get_some_simulation_state(length);
    // let mut state_copy = get_some_flattened_state(length);

    c.bench_function("copy_from", |b| {
        b.iter(|| black_box(state_copy.copy_from(&state)))
        // b.iter(|| black_box(state_copy.copy_from_slice(&state.as_slice())))
    });

    // c.bench_function("copy_physical_state_from", |b| {
    //     b.iter(|| black_box(state_copy.copy_physical_state_from(&state)))
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
