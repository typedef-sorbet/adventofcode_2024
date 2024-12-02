use std::env;
use std::panic;

// auto-import solution functions from files under src/solutions
doko::doko!(
    "src/solutions", 
    "solution", 
    (env::Args) -> Result<i32, &'static str>
);

fn main() {
    use std::env;

    let mut args = env::args();

    if args.len() > 1 {
        let _binname = args.next();
        if let Some(modname) = args.next() {
            // TODO this isn't ideal, maybe submit a PR to doko to add a doko_xyz function that returns a Result?
            if let Ok(modfunc) = panic::catch_unwind(|| doko_solution(modname.as_str())) {
                println!("Running Advent of Code module \"{modname}\"...");
                match modfunc(args) {
                    Ok(res) => println!("Result: {}", res),
                    Err(reason) => println!("Encountered error: {}", reason)
                };
            } else {
                println!("Error: unable to find module \"{modname}\".")
            }
        } else {
            println!("Error: no module name given.");
        }
    } else {
        println!("Error: no module name given.");
    }
}
