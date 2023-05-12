#![allow(non_snake_case)]

fn main() {
    println!();

    for year in std::env::args().skip(1) {
        for day in match year.as_str() {
            "2015" => AdventOfCode::y2015::DAYS,
            "2016" => AdventOfCode::y2016::DAYS,
            "2017" => AdventOfCode::y2017::DAYS,
            "2018" => AdventOfCode::y2018::DAYS,
            "2019" => AdventOfCode::y2019::DAYS,
            "2020" => AdventOfCode::y2020::DAYS,
            "2021" => AdventOfCode::y2021::DAYS,
            "2022" => AdventOfCode::y2022::DAYS,
            _ => &[(|| println!("Unknown year!")) as fn()],
        } {
            day();
        }
    }
}
