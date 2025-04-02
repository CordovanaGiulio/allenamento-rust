use chrono::{Months, NaiveDate};

pub fn stampa_mese(mese : u32, anno : u32){
    let data: NaiveDate;
    match NaiveDate::from_ymd_opt(anno.try_into().unwrap(), mese, 1){
        Some(new) => data=new,
        None => panic!("Data non valida"),
    };
    let last_day_month = ultimo_giorno_mese(&data);
    for giorno in 1..=last_day_month{
        if (giorno-1)%5==0 {
            print!("|\t");
        }
        print!("{}\t", giorno);
        if giorno%5==0 {
            println!("|\t");
        }
    }

}
fn ultimo_giorno_mese(data: &NaiveDate)->i32{
    let data_mese_piu=data.checked_add_months(Months::new(1)).unwrap();

    return data_mese_piu.signed_duration_since(*data).num_days().try_into().unwrap();
}