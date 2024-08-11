use mysql::Value as MySqlValue;
use mysql::Row;

pub fn value_to_string(value: MySqlValue) -> String {
    match value {
        MySqlValue::NULL => "NULL".to_string(),
        MySqlValue::Bytes(bytes) => escape_string(&bytes),
        MySqlValue::Int(int) => int.to_string(),
        MySqlValue::UInt(uint) => uint.to_string(),
        MySqlValue::Float(float) => float.to_string(),
        MySqlValue::Double(double) => double.to_string(),
        MySqlValue::Date(year, month, day, hour, minute, second, micro) => {
            if year == 0 && month == 0 && day == 0 && hour == 0 && minute == 0 && second == 0 && micro == 0 {
                "NULL".to_string()
            } else {
                format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}", year, month, day, hour, minute, second, micro)
            }
        },
        _ => "NULL".to_string(),
    }
}

fn escape_string(bytes: &[u8]) -> String {
    let mut escaped_string = String::from_utf8_lossy(bytes).to_string();
    escaped_string = escaped_string.replace("\\", "\\\\");
    escaped_string = escaped_string.replace("\"", "\\\"");
    escaped_string = escaped_string.replace("\n", "\\n");
    escaped_string = escaped_string.replace("\r", "\\r");
    escaped_string = escaped_string.replace("\t", "\\t");
    escaped_string
}

pub fn to_vector_string(row: Row) -> Vec<String> {
    row.unwrap()
        .into_iter()
        .map(value_to_string)
        .collect()
}
