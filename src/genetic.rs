use crate::physics::Physics;
use rand::Rng;

pub struct GeneticAlgorithm {
    pub population: Vec<Physics>,
    pub mutation_rate: f32,
}

impl GeneticAlgorithm {
    pub fn new(population_size: usize, initial_position: (f32, f32), tick_rate: f32) -> Self {
        let population = (0..population_size)
            .map(|_| {
                let mut physics = Physics::new_with_position(initial_position, tick_rate);
                physics.start();
                physics
            })
            .collect();

        Self {
            population,
            mutation_rate: 0.1,
        }
    }

    pub fn select_best(&self) -> Vec<&Physics> {
        let mut sorted = self.population.iter().collect::<Vec<_>>();
        sorted.sort_by(|a, b| b.get_score().partial_cmp(&a.get_score()).unwrap());
        sorted.into_iter().take(2).collect()
    }

    pub fn crossover(&self, parent1: &Physics, parent2: &Physics) -> Vec<Vec<f32>> {
        let mut rng = rand::thread_rng();
        parent1
            .get_keyframes()
            .iter()
            .zip(parent2.get_keyframes())
            .map(|(f1, f2)| {
                f1.iter()
                    .zip(f2)
                    .map(|(&a, &b)| if rng.gen_bool(0.5) { a } else { b })
                    .collect()
            })
            .collect()
    }

    pub fn mutate(&self, keyframes: &mut Vec<Vec<f32>>) {
        let mut rng = rand::thread_rng();
        for frame in keyframes.iter_mut() {
            for angle in frame.iter_mut() {
                if rng.gen::<f32>() < self.mutation_rate {
                    *angle += rng.gen_range(-10.0..10.0);
                    *angle = angle.clamp(-180.0, 180.0);
                }
            }
        }
    }

    pub fn evolve(&mut self) {
        let best = self.select_best();
        let mut new_population = Vec::new();

        while new_population.len() < self.population.len() {
            let child_keyframes = self.crossover(best[0], best[1]);
            let mut child = Physics::new_with_position((150.0, 345.0), 1.0 / 60.0);
            child.set_keyframes(child_keyframes);
            self.mutate(&mut child.keyframes);
            child.start();
            new_population.push(child);
        }

        self.population = new_population;
    }
}
