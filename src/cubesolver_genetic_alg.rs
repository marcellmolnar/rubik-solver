use rand::Rng;
use crate::rubikscube::RubiksCube;
use crate::common_functions::{rotate_cube_with_moves};

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
}

impl GeneticAlgorithm {
    pub fn new() -> Self {
        GeneticAlgorithm {
            population: Population {
                individuals: Vec::new(),
                size: 200,
            },
            mutation_rate: 0.1,
            crossover_rate: 0.5,
            max_generations: 1000,
        }
    }

    pub fn init_population(&mut self) {
        let mut rng = rand::thread_rng();
        for _i in 0..self.population.size {
            let mut moves: Vec<i32> = Vec::new();
            for _j in 0..40 {
                moves.push(rng.gen_range(0..12));
            }
            self.population.individuals.push(Individual {
                moves: moves,
                fitness: 0,
            });
        }
    }
}

fn evaluate_fitness(cube: &RubiksCube, moves: &Vec<i32>) -> i32 {
    let mut temp_cube = cube.clone();
    rotate_cube_with_moves(&mut temp_cube, moves);
    return temp_cube.state_score();
}

pub fn solve_cube(cube: &mut RubiksCube) -> String {
    let mut ga = GeneticAlgorithm::new();
    ga.init_population();

    for _i in 0..ga.max_generations {
        // evaluate fitness of each individual
        for j in 0..ga.population.size {
            ga.population.individuals[j].fitness = evaluate_fitness(cube, &ga.population.individuals[j].moves);
        }

        // sort them by fitness
        ga.population.individuals.sort_by(|a, b| b.fitness.cmp(&a.fitness));
        
        // crossover
        let mut new_population: Vec<Individual> = Vec::new();
        for _j in 0..ga.population.size {
            let mut rng = rand::thread_rng();
            let parent1 = &ga.population.individuals[rng.gen_range(0..ga.population.size)];
            let parent2 = &ga.population.individuals[rng.gen_range(0..ga.population.size)];
            let mut child_moves: Vec<i32> = Vec::new();
            for k in 0..parent1.moves.len() {
                if rng.gen::<f32>() < ga.crossover_rate {
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

        // mutate
        for j in 0..new_population.len() {
            for k in 0..new_population[j].moves.len() {
                if rand::thread_rng().gen::<f32>() < ga.mutation_rate {
                    new_population[j].moves[k] = rand::thread_rng().gen_range(0..12);
                }
            }
        }

        // evaluate fitness of new population
        for j in 0..new_population.len() {
            new_population[j].fitness = evaluate_fitness(cube, &new_population[j].moves);
        }

        // replace old population with new population one by one if the new individual is better
        for j in 0..ga.population.size {
            if new_population[j].fitness > ga.population.individuals[j].fitness {
                ga.population.individuals[j] = new_population[j].clone();
            }
        }

        // print best 10 individuals score
        for j in 0..10 {
            print!("{:?} ", ga.population.individuals[j].fitness);
        }
        println!("");
    }

    for j in 0..10{
        println!("{} {:?}", ga.population.individuals[j].fitness, ga.population.individuals[j].moves);
    }
    return "".to_string();
}