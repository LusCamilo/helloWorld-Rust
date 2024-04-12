const SECONDS_IN_MINUTES: u32 = 60;
const MINUTES_IN_HOURS: u32 = 60;
const SECONDS_IN_HOURS: u32 = SECONDS_IN_MINUTES * MINUTES_IN_HOURS;

fn main() {

    println!("Hello, World!");

    let total = 30; //tipo u32
    println!("o cara trabalhou {} horas", total);
    {
        let total = total * MINUTES_IN_HOURS ; 
        println!("trabalhou {} minutos", total);
    }
    {
        let total = total * SECONDS_IN_HOURS;
        println!("trabalhou {} segundos", total);
    }
} // Life time

//Drop
