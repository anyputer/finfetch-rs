use console::{Term}; 

use chrono::{Datelike, Timelike, Utc, Weekday};

use std::io::{Error};
use std::{thread, time};

const WHALE: &str = "
                *     *              
              * * * * * *            
            *     * *     *          
                  * *                
                  * *                
            # # # # # # # # #        
#   #     # # # # # # # # # # #      
  #     # # # # # # # # # # @ # #    
  #   # # # # # # # # # # # @ # # #  
  # # # # # # # # # # # # # # # # #  
    # # # # # # # # # # # # # @ # #  
      # # # # # # # # # # # # # @ @  
        # # # # # # # # # # # # # #  
          @ @ # # # # # # # # @ @ @  
            @ @ @ @ @ @ @ @ @ @      
";

fn main() -> Result<(), Error> {
    let term = Term::stdout();

    let lines_to_clear = WHALE.split('\n').count();

    loop {
        let now = Utc::now();

        for (i, line) in WHALE.split('\n').enumerate() {
            match i {
                0 ... 6 => term.write_line(line),
                7 => {
                    term.write_line(
                        &format!("{}Date: {month} {day:02} , {year}", line,
                            month = get_month_name(now.month()),
                            day = now.day(),
                            year = now.year(),
                        )
                    )
                },
                8 => {
                    term.write_line(
                        &format!("{}Weekday: {weekday}", line,
                            weekday = get_weekday_name(now.weekday())
                        )
                    )
                },
                9 => {
                    term.write_line(
                        &format!("{}Time: {hour:02} : {minute:02} : {second:02}", line,
                            hour = now.hour() + 1,
                            minute = now.minute(),
                            second = now.second(),
                        )
                    )
                },
                10 => {
                    term.write_line(
                        &format!("{}Timezone: {timezone}", line,
                            timezone = now.timezone(),
                        )
                    )
                },
                _ => term.write_line(line),
            }.expect("could not write line");
        }

        thread::sleep(time::Duration::from_secs(1));

        term.clear_last_lines(lines_to_clear)
            .expect("could not clear last lines");
    }
}

fn get_month_name(month: u32) -> &'static str {
    match month {
        01 => "January",
        02 => "February",
        03 => "March",
        04 => "April",
        05 => "May",
        06 => "June",
        07 => "July",
        08 => "August",
        09 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _  => unreachable!(),
    }
}

fn get_weekday_name(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    }
}
