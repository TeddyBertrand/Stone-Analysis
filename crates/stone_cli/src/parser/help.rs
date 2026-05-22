use super::mode::ARG_DEFS;

pub fn print_help() {
    println!("Usage : stone_analysis [MODE] [OPTIONS] [ARGS...]\n");
    println!("Modes :");
    for def in ARG_DEFS.iter().filter(|d| d.mode.is_some()) {
        println!("  {:<4}  {:<14}  {}", def.short, def.long, def.help);
    }
    let opts: Vec<_> = ARG_DEFS.iter().filter(|d| d.mode.is_none()).collect();
    if !opts.is_empty() {
        println!("\nOptions :");
        for def in opts {
            println!("  {:<4}  {:<14}  {}", def.short, def.long, def.help);
        }
    }
}
