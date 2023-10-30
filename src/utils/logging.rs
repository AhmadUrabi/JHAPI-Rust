use chrono::{Datelike, Local, Timelike};
use oracle::pool::Pool;
use oracle::sql_type::Timestamp;

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
    let conn = pool.get().unwrap();
    let mut stmt = conn
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
        .build()
        .unwrap();

    stmt.execute(&[
        &username,
        &route,
        &parameters,
        &timestamp,
        &result,
        &token,
        &ip_addr,
        &method
    ])
    .unwrap();
    conn.commit().unwrap();
}

pub fn getTimestamp() -> Timestamp {
    // Get Current timestamp and convert to year, month, day

    let now = Local::now();
    let year = now.year();
    let month = now.month();
    let day = now.day();
    let hour = now.hour();
    let minute = now.minute();
    let second = now.second();
    let nanosecond = now.nanosecond();

    let timestamp = Timestamp::new(
        year as i32,
        month as u32,
        day as u32,
        hour as u32,
        minute as u32,
        second as u32,
        nanosecond as u32,
    );

    timestamp
}
