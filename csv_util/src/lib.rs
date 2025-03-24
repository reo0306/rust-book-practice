pub fn sum_column(csv: &str, column: &str) -> Option<f64> {
    let rows: Vec<&str> = csv.lines().collect();
    let headers: Vec<&str> = rows[0].split(',').collect();
    let column_index = headers.iter().position(|&h| h == column)?;
    let mut sum = 0.0;
    for row in &rows[1..] {
        let fields: Vec<&str> = row.split(',').collect();
        if let Ok(value) = fields[column_index].parse::<f64>() {
            sum += value;
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sum_column() {
        let csv_data = "id,name,price\n1,apple,150.5\n2,banana,200.0\n3,orange,99.5";
        assert_eq!(sum_column(csv_data, "price"), Some(450.0));
    }

    #[test]
    fn it_sum_column_none() {
        let csv_data = "id,name,price\n1,apple,150.5\n2,banana,200.0\n3,orange,99.5";
        assert_eq!(sum_column(csv_data, "weight"), None);
    }
}
