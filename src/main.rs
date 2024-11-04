use martian::time::*;

fn main() {
    let sol = current_sol(false).unwrap();

    println!("Current Mars Sol Date (MSD): {}", sol);

    let mtc = mtc_now(false).unwrap();

    println!("Current Martian Coordinated Time (MTC): {}", mtc);
}
