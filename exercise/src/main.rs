
enum CitySize {
    Town,       // approximate residents: 1_000
    City,       // approximate residents: 10_000
    Metropolis, // approximate residents: 1_000_000
}
#[derive(Debug)]

struct City {
    description: String,
    residents: u64,
    is_coastal: bool,
}

impl City {
    fn new(city_size: CitySize, is_coastal: bool) -> City {
        let (description, residents) = match city_size {
            CitySize::Town => {
                let residents = 1_000;

                (
                    format!("a *town* of approximately {} residents", residents),
                    residents,
                )
            },
            CitySize::City=>{
                let residents =10_000;
                (
                    format!("a *city* of approximately {} residents", residents),
                    residents
                )


            },
            CitySize::Metropolis=>{
                let residents=1_000_000;
                (
                    format!("a *Metropolis* of approximately {} residents", residents),
                    residents 
                )
            }


          
            _ => {
                let residents = 1_000;

                (
                    format!(
                        "an *unknown-size city* of approximately {} residents",
                        residents
                    ),
                    residents,
                )
            }
        };

        City {
            description,
            residents,
            is_coastal,
        }
    }
}
fn main() {
    // 👉 TODO Use City::new() to create a Metropolis-sized city here
    // let rustville = City {
    //     description: String::new(),
    //     residents: 0,
    // };
    let rustville=City::new(CitySize::Town,true);
    let jumanji=City::new(CitySize::City,false);
    let kia_michael=City::new(CitySize::Metropolis,true);

    // println!("This city is {}", rustville.description);
    println!("This city is {:?}", jumanji.description);
    println!("This city is {:?}", kia_michael.description);

    if kia_michael.residents > 100_000 {
        println!("Wow!");
    }
}