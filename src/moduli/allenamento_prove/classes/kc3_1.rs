use chrono::{Datelike, Months, NaiveDate, Utc};

pub fn stampa_mese(data: NaiveDate){
    let giorno_attuale:i32;
    let data_attuale = Utc::now().date_naive();
    if data_attuale == data{
        giorno_attuale=(data.day0()+1).try_into().unwrap();
    }else{
        giorno_attuale=-1;
    }
    let last_day_month = ultimo_giorno_mese(&data);
    let offset =offset_settimana(&data);
    print_all_days();

    print!("|");
    for _ in 1..=offset{
        print!("\t");
    }
    
    for giorno in 1..=last_day_month{
        if giorno!= giorno_attuale{
            print!("\t{}", giorno);
        }else{
            print!("\tX");
        }
        if (offset+giorno)%7==0{
            print!("\t|\n|");
        }
    }
    let mut gg_mese_dopo = last_day_month-1;
    while (offset+gg_mese_dopo)%7!=0{
        print!("\t");
        gg_mese_dopo+=1;
        if (offset+gg_mese_dopo)%7==0{
            print!("|");
        }
    }

}
fn offset_settimana(data: &NaiveDate)->i32{
    let weekday = data.weekday();
    match weekday {
        chrono::Weekday::Mon => return 0,
        chrono::Weekday::Tue => return 1,
        chrono::Weekday::Wed => return 2,
        chrono::Weekday::Thu => return 3,
        chrono::Weekday::Fri => return 4,
        chrono::Weekday::Sat => return 5,
        chrono::Weekday::Sun => return 6,
    }
}
fn ultimo_giorno_mese(data: &NaiveDate)->i32{
    let data_mese_piu=data.checked_add_months(Months::new(1)).unwrap();
    return data_mese_piu.signed_duration_since(*data).num_days().try_into().unwrap();
}
fn print_all_days(){
    print!("|");
    print!("\t{}", chrono::Weekday::Mon);
    print!("\t{}", chrono::Weekday::Tue); 
    print!("\t{}", chrono::Weekday::Wed );
    print!("\t{}", chrono::Weekday::Thu );
    print!("\t{}", chrono::Weekday::Fri );
    print!("\t{}", chrono::Weekday::Sat );
    print!("\t{}", chrono::Weekday::Sun );
    println!("\t|");
}