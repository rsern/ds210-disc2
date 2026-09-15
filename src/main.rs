/// Your crew's name. Both of you are going to change this line.
const CREW_NAME: &str = "the unnamed crew";

/// Your crew's motto. You will both change this one too, earlier and separately.
<<<<<<< HEAD
const MOTTO: &str = "fly high";
>>>>>>> cb5baa53535a308c36c9254f436c34db417642b1

fn main() {
    println!("=== {} ===", CREW_NAME);
    println!();
    println!("Crew roster:");

    // ROSTER: replace the line below with one for yourself.
    println!("  (nobody has signed on yet)");

    println!();
    println!("Motto: {}", MOTTO);
    println!("Report any problems to whoever merged last.");
}
