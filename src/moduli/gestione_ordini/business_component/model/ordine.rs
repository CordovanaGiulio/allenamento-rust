use chrono::NaiveDate;

pub struct Ordine{
    pub username_cli: String,
    pub data_ordine: NaiveDate,
    pub num_prodotti: i32,
    pub prezzo:i32,
}
