use chrono::{FixedOffset, Utc};

pub fn get_ist_date_time() ->  String{
    let ist_offset = FixedOffset::east_opt(19800).unwrap();

    // Convert the current UTC time to IST
    let ist_time = Utc::now().with_timezone(&ist_offset);

    // Format the output
    let formatted = ist_time.format("%Y-%m-%d %H:%M:%S IST").to_string();
    return formatted;
}