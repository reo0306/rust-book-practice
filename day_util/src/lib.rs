use chrono::{NaiveDate, Duration};

pub fn days_between(left: &str, right: &str) -> Option<i64> {
    let dt1 = NaiveDate::parse_from_str(left, "%Y-%m-%d").ok()?;
    let dt2 = NaiveDate::parse_from_str(right, "%Y-%m-%d").ok()?;
    let duration = dt2.signed_duration_since(dt1).num_days();

    Some(duration)
}

pub fn add_days(left: &str, right: i64) -> Option<String> {
    let d1 = NaiveDate::parse_from_str(left, "%Y-%m-%d").ok()?;
    let days = d1.checked_add_signed(Duration::days(right));
    let formatted = days?.format("%Y-%m-%d").to_string();

    Some(formatted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_days_between() {
        let result = days_between("2023-01-01", "2023-12-31");
        assert_eq!(result, Some(364));
    }

    #[test]
    fn it_days_between_err1() {
        let result = days_between("202301-01", "2023-12-31");
        assert_eq!(result, None);
    }

    #[test]
    fn it_days_between_err2() {
        let result = days_between("2023-01-01", "202312-31");
        assert_eq!(result, None);
    }

    #[test]
    fn it_add_days() {
        let result = add_days("2023-01-01", 30);
        assert_eq!(result, Some("2023-01-31".to_string()));
    }

    #[test]
    fn it_add_days_err() {
        let result = add_days("202301-01", 30);
        assert_eq!(result, None);
    }
}
