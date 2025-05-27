use rand::Rng;
use crate::rubikscube::RubiksCube;
use crate::common_functions::{evaluate_score, moves_to_string};

const MOVES_AVAILABLE: i32 = 12;
const MAX_MOVES_ALLOWED: i32 = 20;

#[derive(Clone)]
struct Individual {
    moves: Vec<i32>,
    fitness: i32,
}

struct Population {
    individuals: Vec<Individual>,
    size: usize,
}

struct GeneticAlgorithm {
    population: Population,
    mutation_rate: f32,
    crossover_rate: f32,
    max_generations: i32,
    selection_ratio: f32,
}

impl GeneticAlgorithm {
    pub fn new() -> Self {
        GeneticAlgorithm {
            population: Population {
                individuals: Vec::new(),
                size: 1000,
            },
            mutation_rate: 0.2,
            crossover_rate: 0.9,
            max_generations: 10000,
            selection_ratio: 0.2,
        }
    }

    pub fn init_population(&mut self) {
        let mut rng = rand::thread_rng();
        for _i in 0..self.population.size {
            let mut moves: Vec<i32> = Vec::new();
            for _j in 0..MAX_MOVES_ALLOWED {
                moves.push(rng.gen_range(0..MOVES_AVAILABLE));
            }
            self.population.individuals.push(Individual {
                moves: moves,
                fitness: 0,
            });
        }
    }

    pub fn crossover(&mut self) -> Vec<Individual> {
        //return self.crossover_random();
        return self.crossover_onepoint();
    }

    pub fn crossover_random(&mut self) -> Vec<Individual> {
        let mut new_population: Vec<Individual> = Vec::new();
        for _j in 0..self.population.size {
            let mut rng = rand::thread_rng();
            let parent1_index = rng.gen_range(0..(self.selection_ratio * (self.population.size as f32)) as usize);
            let parent2_index = rng.gen_range(0..((0.9*self.population.size as f32) as usize));
            let parent1 = &self.population.individuals[parent1_index];
            let parent2 = &self.population.individuals[parent2_index];
            let mut child_moves: Vec<i32> = Vec::new();
            for k in 0..parent1.moves.len() {
                if rng.gen::<f32>() < self.crossover_rate {
                    child_moves.push(parent1.moves[k]);
                } else {
                    child_moves.push(parent2.moves[k]);
                }
            }
            new_population.push(Individual {
                moves: child_moves,
                fitness: 0,
            });
        }
        return new_population;
    }

    pub fn crossover_onepoint(&mut self) -> Vec<Individual> {
        let mut new_population: Vec<Individual> = Vec::new();
        for _j in 0..self.population.size {
            let mut rng = rand::thread_rng();
            let parent1_index = rng.gen_range(0..(self.selection_ratio * (self.population.size as f32)) as usize);
            let parent2_index = rng.gen_range(0..((0.9*self.population.size as f32) as usize));
            let parent1 = &self.population.individuals[parent1_index];
            let parent2 = &self.population.individuals[parent2_index];
            let crossover_point = rng.gen_range(0..parent1.moves.len());
            for i in 0..1{
                let mut child_moves: Vec<i32> = Vec::new();
                for k in 0..parent1.moves.len() {
                    if (k < crossover_point && i == 0) || (k > crossover_point && i == 1) {
                        child_moves.push(parent1.moves[k]);
                    } else {
                        child_moves.push(parent2.moves[k]);
                    }
                }
                new_population.push(Individual {
                    moves: child_moves,
                    fitness: 0,
                });
            }
        }
        return new_population;
    }
}

pub fn solve_cube(cube: &mut RubiksCube) -> String {
    let mut ga = GeneticAlgorithm::new();
    ga.init_population();

    for _i in 0..ga.max_generations {
        // evaluate fitness of each individual
        for j in 0..ga.population.size {
            ga.population.individuals[j].fitness = evaluate_score(cube, &ga.population.individuals[j].moves);
        }

        // sort them by fitness
        ga.population.individuals.sort_by(|a, b| b.fitness.cmp(&a.fitness));
        
        if ga.population.individuals[0].fitness == 144 {
            return moves_to_string(&ga.population.individuals[0].moves);
        }

        // crossover
        let mut new_population: Vec<Individual> = ga.crossover();

        // mutate
        for j in 0..new_population.len() {
            for k in 0..new_population[j].moves.len() {
                if rand::thread_rng().gen::<f32>() < ga.mutation_rate {
                    new_population[j].moves[k] = rand::thread_rng().gen_range(0..MOVES_AVAILABLE);
                }
            }
        }

        // evaluate fitness of new population
        for j in 0..new_population.len() {
            new_population[j].fitness = evaluate_score(cube, &new_population[j].moves);
        }

        // replace old population with new population one by one if the new individual is better
        for j in 0..ga.population.size {
            if new_population[j].fitness > ga.population.individuals[j].fitness {
                ga.population.individuals[j] = new_population[j].clone();
            }
        }

        if _i % 50 == 0 {        
            print!("Generation {}: ", _i);
            // print best 10 individuals score
            for j in 0..10 {
                print!("{:?} ", ga.population.individuals[j].fitness);
            }
            println!("");
        }
    }

    for j in 0..10{
        println!("{} {:?}", ga.population.individuals[j].fitness, moves_to_string(&ga.population.individuals[j].moves));
    }
    return "".to_string();
}