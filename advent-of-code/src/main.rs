mod day1;

fn main() -> std::io::Result<()> {
    let result = day1::calculate()?;
    println!("{}", result);
    Ok(())
}
