use rand::Rng;
use std::env;
use std::process;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut program_state = ProgState::new(&args).unwrap_or_else(|err| {
        println!("\nERROR: {}\n\nUSAGE: {}", err, ProgState::get_usage());
        process::exit(1);
    });
    println!("Begin {} iterations with strategy {}", &args[1], &args[2]);
    iterate(&mut program_state);
    print_status(&program_state);
}

fn print_status(program_state: &ProgState) {
    println!("Tested {} of {} iterations", program_state.iterations_performed, program_state.iterations);
    let stay_wins = program_state.stay_wins;
    let switch_wins = program_state.iterations_performed - program_state.stay_wins;
    let test_count = program_state.iterations_performed;

    match program_state.strategy {
        Strategy::STAY => print_strat_status("STAY", stay_wins, test_count),
        Strategy::SWITCH => print_strat_status("SWITCH", switch_wins, test_count),
        Strategy::BOTH => {
            print_strat_status("STAY", stay_wins, test_count);
            print_strat_status("SWITCH", switch_wins, test_count);
        },
    }
}

fn print_strat_status(strategy_string: &str, wins: u128, iterations: u128) {
    let winpercent = 100.0 * ((wins as f64)/(iterations as f64));
    println!("\t{} wins {} times (win rate: {}%)",strategy_string, wins, winpercent);
}

fn iterate(program_state: &mut ProgState) {
    let mut rng = rand::thread_rng();
    let mut now = Instant::now();
    while program_state.iterations_performed < program_state.iterations {
        program_state.iterations_performed = program_state.iterations_performed + 1;
        let mut goat_doors = vec![0, 1, 2];
        let car_door = rng.gen_range(0,3);
        goat_doors.remove(car_door);
        let contestant_door = rng.gen_range(0,3);

        let open_door;
        if contestant_door == goat_doors[0] {
            open_door = goat_doors[1];
        } else if contestant_door == goat_doors[1] {
            open_door = goat_doors[0];
        } else {
            //contestant door isn't a goat door.  Means it wins on stay
            program_state.stay_wins = program_state.stay_wins + 1;
            open_door = goat_doors[rng.gen_range(0,2)];
        }

        if program_state.logging {
            println!("Test {} of {}:\n\tCar behind {}, Goats behind {} and {}\n\t\
                     Contestant Chooses {}\n\t\
                     Monty Opens {} and reveals a goat", 
                     program_state.iterations_performed, 
                     program_state.iterations, 
                     car_door + 1, 
                     goat_doors[0] + 1, 
                     goat_doors[1] + 1, 
                     contestant_door + 1,
                     open_door + 1);
        } else if now.elapsed().as_millis() >= 1000 {
            print_status(&program_state);
            now = Instant::now();
        }
    }
}

struct ProgState {
    iterations: u128,
    iterations_performed: u128,
    stay_wins: u128,
    logging: bool,
    strategy: Strategy,
}

enum Strategy {
    STAY,
    SWITCH,
    BOTH,
}

impl ProgState {
    fn new(args: &[String]) -> Result<ProgState, &'static str> {
    
        if args.len() != 3 && args.len() != 4 {
            return Err("Incomplete argument list");
        }

        let iterations = match u128::from_str_radix(&args[1], 10) {
            Ok(count) => count,
            Err(_) => return Err("Iterations argument failed to parse"),
        };
        if iterations == 0 {
            return Err("Zero is an invalid number of iterations");
        }

        let strategy = match args[2].as_str() {
            "STAY" => Strategy::STAY,
            "SWITCH" => Strategy::SWITCH,
            "BOTH" => Strategy::BOTH,
            _ => return Err("Strategy failed to parse"),
        };
        
        let logging;
        if args.len() == 4 && args[3].as_str() == "DEBUGLOG" {
            logging = true;
        } else {
            logging = false;
        }

        Ok(ProgState { iterations, iterations_performed: 0, stay_wins: 0, logging, strategy })
    }
    fn get_usage() -> &'static str {
        return "montyhall [iterations] [strategy]\n\t \
            Iterations:\tNumber of tests to run (1 or more)\n\t \
            Strategy:\tChoose between STAY, SWTICH, or BOTH\n\t \
            Logging:\tOptional.  Enter DEBUGLOG to enable"
    }
}
