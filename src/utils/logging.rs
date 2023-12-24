use chrono::{Datelike, Local, Timelike};
use oracle::pool::Pool;
use oracle::sql_type::Timestamp;


// TODO: fix this mess
pub fn log_data(
    pool: &Pool,
    username: String,
    ip_addr: String,
    route: String,
    parameters: Option<String>,
    timestamp: Timestamp,
    token: String,
    result: String,
    method: String
) {
    let conn = pool.get();
    if conn.is_err() {
        error!("Error: {}", conn.err().unwrap());
        return;
    }
    let conn = conn.unwrap();

    let stmt = conn
        .statement(
            "
        INSERT INTO odbc_jhc.API_LOGS (
            username,
            route,
            parameters,
            timestamp,
            result,
            token_used,
            ip_address,
            method
        ) VALUES (
            :username,
            :route,
            :parameters,
            :timestamp,
            :result,
            :token_used,
            :ip_address,
            :method
        )",
        )
        .build();
    if stmt.is_err() {
        error!("Error building statement: {}", stmt.err().unwrap());
        return;
    }
    let mut stmt = stmt.unwrap();

    match stmt.execute(&[
        &username,
        &route,
        &parameters,
        &timestamp,
        &result,
        &token,
        &ip_addr,
        &method
    ]) {
        Ok(_) => (),
        Err(err) => {
            error!("Error executing query: {}", err);
            return;
        }
    };
    match conn.commit() {
        Ok(_) => (),
        Err(err) => {
            error!("Error: {}", err);
            return;
        }
    }
}

pub fn get_timestamp() -> Timestamp {
    // Get Current timestamp and convert to year, month, day
    let now = Local::now();

    let timestamp = Timestamp::new(
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
        now.nanosecond(),
    );

    timestamp
}
