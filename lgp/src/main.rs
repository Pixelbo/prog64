use std::process::exit;

use env::{cartpole::CartPole, Environment, pcgrng::PCGRng};

use crate::{interpretor::Machine, mutation::{Mutator, UniformMutation}, progamme::Programme, selector::{Selector, Truncation}};


pub mod interpretor;
pub mod progamme;
pub mod selector;

pub mod mutation;
const NB_GENERATIONS: usize = 100;
const NB_POPULATION: usize = 100;



fn main() {


    //Init the population
    let mut population: [Programme; NB_POPULATION] = std::array::from_fn( |i|{
        Programme::default()
    });

    //Select the method of selection & mutation
    let mut selection_method = Truncation{portion:0.1};
    let mut mutator = UniformMutation { taux: 0.5 };

    //Todo: 2 boulces NB_GEN * NB_POP
    //Evolution is Action -> Selection -> Mutation (repopulation) -> .
    //ACTION: One generation loop
    for curr_prog in &mut population {

        //Init the env and the cart
        let seed = 1;
        let mut rng = PCGRng::seed(seed);
        let mut env = CartPole::default();
        let mut total_reward = 0.0;

        //Reset the score of the program between generations 
        curr_prog.score = 0.;

        //Init the machine (that hold the registers) and the program
        let mut machine = Machine {
            r: [0., 0., 0., 0.],
            i: [0., 0., 0., 0.],
        };

        //The first action is random
        let mut action: f32 = (rng.next_u32() % 4) as f32;
        loop {
            let (curr_state, reward, done) = env.step(action, &mut rng);

            machine.i = curr_state; //Copy the inputs
            machine.executer_programme(curr_prog);

            action = (machine.r[0]) as f32;

            total_reward += reward;
            if done {
                break;
            }
        }

        curr_prog.score += total_reward;

    }
    //SELECTION
    
    let selected = match selection_method.select(&mut population){
        Ok(pop_select) => pop_select,
        Err(error) => panic!("Problem selecting the population: {:?}", error),
    };
    //REPOPULATION / MUTATION
    let nb_pop_after_select = selected.len();

    for i in 0..nb_pop_after_select{
        population[i] = selected[i%nb_pop_after_select].clone();
    }

    for i in nb_pop_after_select+1..NB_POPULATION{
        population[i] = Programme::default();
        //Mutate the best of the best

        let mutated_prog = mutator.mutate(&mut population[i%nb_pop_after_select]).unwrap();

        population[i] = mutated_prog.clone();
    }
    
    println!("{:?}", population);
}
