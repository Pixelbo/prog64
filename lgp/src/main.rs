use std::process::exit;

use env::{cartpole::CartPole, Environment, pcgrng::PCGRng};

use crate::progamme::Programme;


pub mod interpretor;
pub mod progamme;
const NB_GENERATIONS: usize = 100;
const NB_POPULATION: usize = 100;



fn main() {
    let seed = 1;
    let mut rng = PCGRng::seed(seed);
    let mut env = CartPole::default();
    let mut total_reward = 0.0;

    let mut test_prog = Programme::init("MyTest".to_string(), 0b1110001101);
    println!("{:?}", test_prog);
    println!("{:?}", test_prog.getOut(0));
    println!("{:?}", test_prog.getOp(0));
    println!("{:?}", test_prog.getInA(0));
    println!("{:?}", test_prog.getInB(0));
    



    exit(0);
    //Init the population
    let mut population: [Programme; NB_POPULATION] = std::array::from_fn( |i|{
        Programme::default()
    });


    let mut iterations = 0;

    println!("I\taction\treward\tdone\tx\tx_dot\ttheta\ttheta_dot");
    //Todo: 2 boulces NB_GEN * NB_POP
    loop {
        
        let action: f32 = (rng.next_u32() % 4) as f32;

        
        let (curr_state, reward, done) = env.step(action, &mut rng);

        println!("{}\t{}\t{}\t{}\t{:.2}\t{:.2}\t{:.2}\t{:.2}",iterations ,action, reward, done, curr_state[0], curr_state[1], curr_state[2], curr_state[3]);

        total_reward += reward;
        iterations+=1;
        if done {
            break;
        }
    }
    //Faire logging et recap
    println!("Total reward: {}", total_reward)
}
