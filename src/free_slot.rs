use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

#[derive(Serialize, Deserialize)]
struct Calendar {
    events: Vec<(String, String)>,
    not_before: String,
    not_after: String,
}

impl Calendar {
    fn new(events: Vec<(String, String)>, not_before: String, not_after: String) -> Calendar {
        Calendar {
            events,
            not_before,
            not_after,
        }
    }
}

struct TimeStore {
    time_store: Vec<bool>,
}

impl TimeStore {
    fn new(size: usize) -> TimeStore {
        let mut tmp: Vec<bool> = Vec::with_capacity(size);
        for _ in 0..size {
            tmp.push(true);
        }

        TimeStore { time_store: tmp }
    }

    fn update(&mut self, cal_a: &[bool]) {
        for (store, update) in self.time_store.iter_mut().zip(cal_a.iter()) {
            *store &= !update;
        }
    }
}

fn parse_time(time_stamp: &str) -> std::io::Result<usize> {
    let mut output = 0;
    for token in time_stamp.split(":") {
        let tmp: usize = parse_int(token)?;
        output *= 60;
        output += tmp;
    }

    if output >= (24 * 60) {
        let err = Error::new(ErrorKind::Other, format!("{} out of range [0-{}[", 0, 24 * 60));
        Err(err)
    } else {
        Ok(output)
    }
    
}

fn parse_int(token: &str) -> std::io::Result<usize> {
    match token.parse() {
        Ok(n) => Ok(n),
        Err(parse_err) => {
            let err = Error::new(ErrorKind::Other, format!("{}", parse_err));
            Err(err)
        }
    }
}

fn set_occupied(begin: usize, end: usize, day_calendar: &mut [bool]) {
    for i in begin..end {
        day_calendar[i] = true;
    }
}

fn calendar_generator(cal: &Calendar) -> std::io::Result<[bool; 24 * 60]> {
    let mut output = [false; 24 * 60];

    for event in cal.events.iter() {
        let begin = parse_time(&event.0)?;
        let end = parse_time(&event.1)?;

        set_occupied(begin, end, &mut output);
    }

    let not_before = parse_time(&cal.not_before)?;
    set_occupied(0, not_before, &mut output);

    let not_after = parse_time(&cal.not_after)?;
    set_occupied(not_after, 24 * 60, &mut output);

    Ok(output)
}

fn int_to_time_stamp(minute: usize) -> String {
    let h = minute / 60;
    let m = minute % 60;
    format!("{:02}:{:02}", h, m)
}

fn update_free_slots(begin: usize, end: usize, slots: &mut Vec<(String, String)>) {
    let a = int_to_time_stamp(begin);
    let b = int_to_time_stamp(end);
    slots.push((a, b));
}

fn find_free_slots(calendars: &[Calendar]) -> std::io::Result<Calendar> {
    let mut time_store = TimeStore::new(24 * 60);
    for cal in calendars.iter() {
        let tmp = calendar_generator(cal)?;
        time_store.update(&tmp);
    }

    let mut output = Vec::new();
    let mut state = false;
    let mut begin: usize = 0;
    for (index, value) in time_store.time_store.iter().enumerate() {
        if *value {
            if !state {
                begin = index;
                state = true;
            }
        } else {
            if state {
                update_free_slots(begin, index, &mut output);
                state = false;
            }
        }
    }

    let out = Calendar::new(output, "00:00".to_owned(), "23:59".to_owned());
    Ok(out)
}

pub fn free_slots(json_cal: &str) -> Result<String, Error> {
    let calendars: Vec<Calendar> = serde_json::from_str(json_cal)?;

    let ans = find_free_slots(&calendars)?;
    let output = serde_json::to_string(&ans)?;
    Ok(output)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parse_time() {
        let hours = [4, 7, 10, 16];
        let minutes = [0, 12, 45, 23];
        for h in hours.iter() {
            for m in minutes.iter() {
                let time = format!("{:02}:{:02}", h, m);
                let ans = (h * 60 + m) as usize;
                assert_eq!(parse_time(&time).unwrap(), ans);
            }
        }
    }

    #[test]
    fn test_int_to_time_stamp() {
        let hours = [4, 7, 10, 16];
        let minutes = [0, 12, 45, 23];
        for h in hours.iter() {
            for m in minutes.iter() {
                let ans = format!("{:02}:{:02}", h, m);
                let time = (h * 60 + m) as usize;
                assert_eq!(int_to_time_stamp(time), ans);
            }
        }
    }

    #[test]
    fn test_find_free_time_slots() {
        let cal_a = Calendar::new(
            vec![("11:30".to_owned(), "12:30".to_owned())],
            "9:30".to_owned(),
            "13:45".to_owned(),
        );
        let cal_b = Calendar::new(
            vec![("8:45".to_owned(), "13:00".to_owned())],
            "6:00".to_owned(),
            "21:00".to_owned(),
        );
        let cal_c = Calendar::new(
            vec![
                ("8:45".to_owned(), "11:35".to_owned()),
                ("13:30".to_owned(), "17:45".to_owned()),
            ],
            "6:00".to_owned(),
            "21:00".to_owned(),
        );

        let calendars = [cal_a, cal_b, cal_c];

        let ans = find_free_slots(&calendars).unwrap();
        let correct = vec![("13:00".to_owned(), "13:30".to_owned())];
        assert_eq!(ans.events, correct);
    }

    #[test]
    fn test_calendar_generator() {
        let cal_str = vec![(String::from("12:00"), String::from("14:00"))];
        let cal_int: (usize, usize) = (12 * 60, 14 * 60);
        let not_before_str = "8:00".to_owned();
        let not_before_int: usize = 60 * 8;
        let not_after_str = "17:00".to_owned();
        let not_after_int: usize = 17 * 60;

        let cal = Calendar::new(cal_str, not_before_str, not_after_str);

        let ans = calendar_generator(&cal).unwrap();

        check_value(&ans, 0, not_before_int, true);
        check_value(&ans, not_before_int, cal_int.0, false);
        check_value(&ans, cal_int.0, cal_int.1, true);
        check_value(&ans, cal_int.1, not_after_int, false);
        check_value(&ans, not_after_int, 24 * 60, true);
    }


    #[test]
    fn test_convertion_error() {
        let ans = parse_time("12:5r");
        assert!(check_error(ans, true));

        let ans = parse_time("123:67");
        assert!(check_error(ans, true));

        let ans = parse_time("23:59");
        assert!(check_error(ans, false));
    
        let ans = parse_time("0:0");
        assert!(check_error(ans, false));
    }

    fn check_error(ans: std::io::Result<usize>, stat: bool) -> bool {
        match ans {
            Ok(_) => !stat,
            Err(_) => stat
        }
    }

    fn check_value(cal: &[bool], from: usize, to: usize, value: bool) {
        for (i, v) in cal[from..to].iter().enumerate() {
            assert_eq!(
                *v, value,
                "Fail interval ({} {}), should be {} @ {}",
                from, to, value, i
            );
        }
    }
}
