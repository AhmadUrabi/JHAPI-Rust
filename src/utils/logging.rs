use chrono::{Datelike, Local, Timelike};
use oracle::pool::Pool;
use oracle::sql_type::Timestamp;

// TODO: fix this mess
pub fn log_data(
    pool: &Pool,
    mut username: String,
    mut ip_addr: String,
    mut route: String,
    mut parameters: Option<String>,
    timestamp: Timestamp,
    mut token: String,
    mut result: String,
    mut method: String,
) {
    let conn = pool.get();
    if conn.is_err() {
        error!("Error: {}", conn.err().unwrap());
        return;
    }
    let conn = conn.unwrap();

    // Chop long VALUES
    // Only really applies for extra long parameters and routes (User Input)
    // TODO: Add a flag for chopped values
    if username.len() > 64 {
        username = username[..64].to_string();
    }
    if ip_addr.len() > 60 {
        ip_addr = ip_addr[..60].to_string();
    }
    if route.len() > 64 {
        route = route[..64].to_string();
    }
    if token.len() > 255 {
        token = token[..255].to_string();
    }
    if result.len() > 200 {
        result = result[..200].to_string();
    }
    if method.len() > 64 {
        method = method[..64].to_string();
    }
    if parameters.is_some() {
        if parameters.as_ref().unwrap().len() > 2000 {
            parameters = Some(parameters.as_ref().unwrap()[..2000].to_string());
        }
    }

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
        &method,
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
