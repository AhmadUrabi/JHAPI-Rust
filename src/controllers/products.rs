#[allow(non_snake_case)]
use std::collections::HashSet;

use oracle::pool::Pool;
use oracle::Row;

use oracle::sql_type::ToSql;
use rocket::log::private::info;
use rocket::serde::json::Json;

use crate::controllers::stores::get_stores;

use crate::server::request_guard::api_key::ApiKey;

use crate::controllers::auth::decode_token_data;
use crate::utils::permissions::is_cost_perm;
use crate::utils::sql::SQLManager;
use crate::utils::structs::APIError;

use rocket::serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Product {
    pub ITEM_ID: Option<String>,
    pub IS_ACTIVE: Option<String>,
    pub CAN_BE_SOLD: Option<String>,
    pub ITEM_DESC: Option<String>,
    pub ITEM_DESC_S: Option<String>,
    pub FOREIGN_ITEM_CODE: Option<String>,
    pub ITEM_CAT: Option<String>,
    pub ITEM_SUB_CAT: Option<String>,
    pub SALE_UNIT: Option<String>,
    pub UNIT_DESC: Option<String>,
    pub PACKING: Option<String>,
    pub CARD_OPEN_DATE: Option<String>,
    pub HS_CODE: Option<String>,
    pub COUNTRY: Option<String>,
    pub COUNTRY_DESC: Option<String>,
    pub SUPPLIER_ID: Option<String>,
    pub SUPPLIER_DESC: Option<String>,
    pub ITEM_MAIN_BARCODE: Option<String>,
    pub NATURE_ID: Option<String>,
    pub NATURE_DESC: Option<String>,
    pub TRADE_ID: Option<String>,
    pub TRADE_DESC: Option<String>,
    pub QTY_STORE_01: Option<String>,
    pub QTY_STORE_02: Option<String>,
    pub QTY_STORE_05: Option<String>,
    pub QTY_STORE_06: Option<String>,
    pub QTY_STORE_07: Option<String>,
    pub QTY_STORE_08: Option<String>,
    pub QTY_STORE_09: Option<String>,
    pub QTY_STORE_10: Option<String>,
    pub QTY_STORE_11: Option<String>,
    pub QTY_STORE_12: Option<String>,
    pub QTY_STORE_19: Option<String>,
    pub QTY_STORE_21: Option<String>,
    pub QTY_STORE_23: Option<String>,
    pub QTY_STORE_31: Option<String>,
    pub QTY_STORE_32: Option<String>,
    pub QTY_STORE_33: Option<String>,
    pub QTY_STORE_34: Option<String>,
    pub QTY_STORE_35: Option<String>,
    pub SALE_PRICE_NOTAX_STORE_01: Option<String>,
    pub SALE_PRICE_NOTAX_STORE_02: Option<String>,
    pub SALE_PRICE_NOTAX_STORE_05: Option<String>,
    pub SALE_PRICE_NOTAX_STORE_06: Option<String>,
    pub SALE_PRICE_NOTAX_STORE_08: Option<String>,
    pub SALE_PRICE_NOTAX_STORE_09: Option<String>,
    pub SALE_PRICE_NOTAX_STORE_07: Option<String>,
    pub SALE_PRICE_NOTAX_STORE_31: Option<String>,
    pub SALE_PRICE_NOTAX_STORE_32: Option<String>,
    pub SALE_PRICE_NOTAX_STORE_33: Option<String>,
    pub SALE_PRICE_NOTAX_STORE_34: Option<String>,
    pub SALE_PRICE_NOTAX_STORE_35: Option<String>,
    pub FIRST_DISC_PER_STORE_01: Option<String>,
    pub FIRST_DISC_PER_STORE_02: Option<String>,
    pub FIRST_DISC_PER_STORE_05: Option<String>,
    pub FIRST_DISC_PER_STORE_06: Option<String>,
    pub FIRST_DISC_PER_STORE_07: Option<String>,
    pub FIRST_DISC_PER_STORE_08: Option<String>,
    pub FIRST_DISC_PER_STORE_09: Option<String>,
    pub FIRST_DISC_PER_STORE_31: Option<String>,
    pub FIRST_DISC_PER_STORE_32: Option<String>,
    pub FIRST_DISC_PER_STORE_33: Option<String>,
    pub FIRST_DISC_PER_STORE_34: Option<String>,
    pub FIRST_DISC_PER_STORE_35: Option<String>,
    pub SECOND_DISC_PER_STORE_01: Option<String>,
    pub SECOND_DISC_PER_STORE_02: Option<String>,
    pub SECOND_DISC_PER_STORE_05: Option<String>,
    pub SECOND_DISC_PER_STORE_06: Option<String>,
    pub SECOND_DISC_PER_STORE_07: Option<String>,
    pub SECOND_DISC_PER_STORE_08: Option<String>,
    pub SECOND_DISC_PER_STORE_09: Option<String>,
    pub SECOND_DISC_PER_STORE_31: Option<String>,
    pub SECOND_DISC_PER_STORE_32: Option<String>,
    pub SECOND_DISC_PER_STORE_33: Option<String>,
    pub SECOND_DISC_PER_STORE_34: Option<String>,
    pub SECOND_DISC_PER_STORE_35: Option<String>,
    pub T_AVE_COST: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Clone, PartialEq)]
pub struct FetchParams {
    pub p_ref: Option<String>,
    pub p_barcode: Option<String>,
    pub p_id: Option<String>,
    pub p_desc: Option<String>,
}

impl FetchParams {
    pub fn is_none(&self) -> bool {
        self.p_ref.is_none()
            && self.p_barcode.is_none()
            && self.p_id.is_none()
            && self.p_desc.is_none()
    }
}


#[allow(unused_assignments)]
pub async fn get_product(
    params: Json<FetchParams>,
    pool: &Pool,
    sql_manager: &SQLManager,
    key: &ApiKey<'_>,
) -> Result<Vec<Product>, APIError> {
    // Empty params are not an error, but they should return an empty vec
    if params.is_none() {
        println!("Empty params");
        return Ok(Vec::new());
    }

    // Get username from token
    let username: String;

    match decode_token_data(key.0) {
        Some(data) => {
            username = data.USER_ID.unwrap().to_string(); // Safe Unwrap, Username is never null in DB
        }
        None => {
            return Err(APIError::InvalidToken);
        }
    }

    // To ensure user only gets data for stores they have access to
    // This does have a performance penalty, as the function has to touch the DB twice

    let mut store_ids: HashSet<String> = HashSet::new();

    match get_stores(pool, &sql_manager, username).await {
        Ok(store_list) => {
            for i in store_list {
                store_ids.insert(i.STORE_ID.unwrap());
            }
        }
        Err(e) => {
            info!("Error getting stores");
            return Err(e);
        }
    }

    // To call the function once, otherwise will call on each product found, and touch DB every time
    let show_cost = is_cost_perm(key, pool, &sql_manager).await;

    // Helper function to get value from row and check if store is in store_ids
    fn get_value(
        store_ids: &HashSet<String>,
        row: &Row,
        store_id: &str,
        column_name: &str,
    ) -> Option<String> {
        if store_ids.contains(&store_id.to_string()) {
            row.get(column_name).ok()
        } else {
            None
        }
    }

    fn add_param(sql: &mut String, param_count: &mut i32, column_name: &str, param: &str) {
        if *param_count > 0 {
            sql.push_str(" AND");
        }
        *param_count += 1;
        if param.starts_with("%") || param.ends_with("%") {
            sql.push_str(&format!(" {} LIKE :{}", column_name, column_name));
        } else {
            sql.push_str(&format!(" {} = :{}", column_name, column_name));
        }
    }

    let mut sql = String::from("SELECT * FROM ODBC_JHC.JHC_INVDATA WHERE");
    let mut my_params: Vec<(&str, &dyn ToSql)> = Vec::new();
    let mut param_count = 0;

    if let Some(p_ref) = &params.p_ref {
        add_param(&mut sql, &mut param_count, "FOREIGN_ITEM_CODE", p_ref);
        my_params.push(("FOREIGN_ITEM_CODE", p_ref as &dyn ToSql));
    }

    if let Some(p_id) = &params.p_id {
        add_param(&mut sql, &mut param_count, "ITEM_ID", p_id);
        my_params.push(("ITEM_ID", p_id as &dyn ToSql));
    }

    if let Some(p_desc) = &params.p_desc {
        add_param(&mut sql, &mut param_count, "ITEM_DESC_S", p_desc);
        my_params.push(("ITEM_DESC_S", p_desc as &dyn ToSql));
    }

    if let Some(p_barcode) = &params.p_barcode {
        if param_count > 0 {
            sql.push_str(" AND");
        }
        param_count += 1;
        if p_barcode.starts_with("%") || p_barcode.ends_with("%") {
            sql.push_str(" BARCODE_LISTED LIKE '%' || :barcode");
        } else {
            param_count += 1;
            sql.push_str(" BARCODE_LISTED LIKE '%' || :barcode || '%'");
        }
        my_params.push(("barcode", p_barcode as &dyn ToSql));
    }

    info!("SQL Statement: {:?}", sql);

    let conn = pool.get().map_err(|e| {
        error!("Connection Error: {:?}", e);
        APIError::DBError
    })?;

    let now = tokio::time::Instant::now();

    let mut stmt = conn.statement(&sql).build().map_err(|e| {
        error!("Error building statement: {:?}", e);
        APIError::DBError
    })?;

    let rows = stmt.query_named(&my_params).map_err(|e| {
        error!("Error executing query: {:?}", e);
        APIError::DBError
    })?;

    println!("Total Query Time: {:?}", now.elapsed().as_millis());

    let mut products: Vec<Product> = Vec::new();

    for row_result in rows {
        if row_result.is_err() {
            info!("Row Error");
            return Err(APIError::DBError);
        }
        let row = row_result.unwrap();
        products.push(Product {
            ITEM_ID: row.get("ITEM_ID").unwrap(),
            IS_ACTIVE: row.get("IS_ACTIVE").unwrap(),
            CAN_BE_SOLD: row.get("CAN_BE_SOLD").unwrap(),
            ITEM_DESC: row.get("ITEM_DESC").unwrap(),
            ITEM_DESC_S: row.get("ITEM_DESC_S").unwrap(),
            FOREIGN_ITEM_CODE: row.get("FOREIGN_ITEM_CODE").unwrap(),
            ITEM_CAT: row.get("ITEM_CAT").unwrap(),
            ITEM_SUB_CAT: row.get("ITEM_SUB_CAT").unwrap(),
            SALE_UNIT: row.get("SALE_UNIT").unwrap(),
            UNIT_DESC: row.get("UNIT_DESC").unwrap(),
            PACKING: row.get("PACKING").unwrap(),
            CARD_OPEN_DATE: row.get("CARD_OPEN_DATE").unwrap(),
            HS_CODE: row.get("HS_CODE").unwrap(),
            COUNTRY: row.get("COUNTRY").unwrap(),
            COUNTRY_DESC: row.get("COUNTRY_DESC").unwrap(),
            SUPPLIER_ID: row.get("SUPPLIER_ID").unwrap(),
            SUPPLIER_DESC: row.get("SUPPLIER_DESC").unwrap(),
            ITEM_MAIN_BARCODE: row.get("ITEM_MAIN_BARCODE").unwrap(),
            NATURE_ID: row.get("NATURE_ID").unwrap(),
            NATURE_DESC: row.get("NATURE_DESC").unwrap(),
            TRADE_ID: row.get("TRADE_ID").unwrap(),
            TRADE_DESC: row.get("TRADE_DESC").unwrap(),
            QTY_STORE_01: get_value(&store_ids, &row, "01", "QTY_STORE_01"),
            QTY_STORE_02: get_value(&store_ids, &row, "02", "QTY_STORE_02"),
            QTY_STORE_05: get_value(&store_ids, &row, "05", "QTY_STORE_05"),
            QTY_STORE_06: get_value(&store_ids, &row, "06", "QTY_STORE_06"),
            QTY_STORE_07: get_value(&store_ids, &row, "07", "QTY_STORE_07"),
            QTY_STORE_08: get_value(&store_ids, &row, "08", "QTY_STORE_08"),
            QTY_STORE_09: get_value(&store_ids, &row, "09", "QTY_STORE_09"),
            QTY_STORE_10: get_value(&store_ids, &row, "10", "QTY_STORE_10"),
            QTY_STORE_11: get_value(&store_ids, &row, "11", "QTY_STORE_11"),
            QTY_STORE_12: get_value(&store_ids, &row, "12", "QTY_STORE_12"),
            QTY_STORE_19: get_value(&store_ids, &row, "19", "QTY_STORE_19"),
            QTY_STORE_21: get_value(&store_ids, &row, "21", "QTY_STORE_21"),
            QTY_STORE_23: get_value(&store_ids, &row, "23", "QTY_STORE_23"),
            QTY_STORE_31: get_value(&store_ids, &row, "31", "QTY_STORE_31"),
            QTY_STORE_32: get_value(&store_ids, &row, "32", "QTY_STORE_32"),
            QTY_STORE_33: get_value(&store_ids, &row, "33", "QTY_STORE_33"),
            QTY_STORE_34: get_value(&store_ids, &row, "34", "QTY_STORE_34"),
            QTY_STORE_35: get_value(&store_ids, &row, "35", "QTY_STORE_35"),
            SALE_PRICE_NOTAX_STORE_01: get_value(
                &store_ids,
                &row,
                "01",
                "SALE_PRICE_NOTAX_STORE_01",
            ),
            SALE_PRICE_NOTAX_STORE_02: get_value(
                &store_ids,
                &row,
                "02",
                "SALE_PRICE_NOTAX_STORE_02",
            ),
            SALE_PRICE_NOTAX_STORE_05: get_value(
                &store_ids,
                &row,
                "05",
                "SALE_PRICE_NOTAX_STORE_05",
            ),
            SALE_PRICE_NOTAX_STORE_06: get_value(
                &store_ids,
                &row,
                "06",
                "SALE_PRICE_NOTAX_STORE_06",
            ),
            SALE_PRICE_NOTAX_STORE_08: get_value(
                &store_ids,
                &row,
                "08",
                "SALE_PRICE_NOTAX_STORE_08",
            ),
            SALE_PRICE_NOTAX_STORE_09: get_value(
                &store_ids,
                &row,
                "09",
                "SALE_PRICE_NOTAX_STORE_09",
            ),
            SALE_PRICE_NOTAX_STORE_07: get_value(
                &store_ids,
                &row,
                "07",
                "SALE_PRICE_NOTAX_STORE_07",
            ),
            SALE_PRICE_NOTAX_STORE_31: get_value(
                &store_ids,
                &row,
                "31",
                "SALE_PRICE_NOTAX_STORE_31",
            ),
            SALE_PRICE_NOTAX_STORE_32: get_value(
                &store_ids,
                &row,
                "32",
                "SALE_PRICE_NOTAX_STORE_32",
            ),
            SALE_PRICE_NOTAX_STORE_33: get_value(
                &store_ids,
                &row,
                "33",
                "SALE_PRICE_NOTAX_STORE_33",
            ),
            SALE_PRICE_NOTAX_STORE_34: get_value(
                &store_ids,
                &row,
                "34",
                "SALE_PRICE_NOTAX_STORE_34",
            ),
            SALE_PRICE_NOTAX_STORE_35: get_value(
                &store_ids,
                &row,
                "35",
                "SALE_PRICE_NOTAX_STORE_35",
            ),
            FIRST_DISC_PER_STORE_01: get_value(&store_ids, &row, "01", "FIRST_DISC_PER_STORE_01"),
            FIRST_DISC_PER_STORE_02: get_value(&store_ids, &row, "02", "FIRST_DISC_PER_STORE_02"),
            FIRST_DISC_PER_STORE_05: get_value(&store_ids, &row, "05", "FIRST_DISC_PER_STORE_05"),
            FIRST_DISC_PER_STORE_06: get_value(&store_ids, &row, "06", "FIRST_DISC_PER_STORE_06"),
            FIRST_DISC_PER_STORE_07: get_value(&store_ids, &row, "07", "FIRST_DISC_PER_STORE_07"),
            FIRST_DISC_PER_STORE_08: get_value(&store_ids, &row, "08", "FIRST_DISC_PER_STORE_08"),
            FIRST_DISC_PER_STORE_09: get_value(&store_ids, &row, "09", "FIRST_DISC_PER_STORE_09"),
            FIRST_DISC_PER_STORE_31: get_value(&store_ids, &row, "31", "FIRST_DISC_PER_STORE_31"),
            FIRST_DISC_PER_STORE_32: get_value(&store_ids, &row, "32", "FIRST_DISC_PER_STORE_32"),
            FIRST_DISC_PER_STORE_33: get_value(&store_ids, &row, "33", "FIRST_DISC_PER_STORE_33"),
            FIRST_DISC_PER_STORE_34: get_value(&store_ids, &row, "34", "FIRST_DISC_PER_STORE_34"),
            FIRST_DISC_PER_STORE_35: get_value(&store_ids, &row, "35", "FIRST_DISC_PER_STORE_35"),
            SECOND_DISC_PER_STORE_01: get_value(&store_ids, &row, "01", "SECOND_DISC_PER_STORE_01"),
            SECOND_DISC_PER_STORE_02: get_value(&store_ids, &row, "02", "SECOND_DISC_PER_STORE_02"),
            SECOND_DISC_PER_STORE_05: get_value(&store_ids, &row, "05", "SECOND_DISC_PER_STORE_05"),
            SECOND_DISC_PER_STORE_06: get_value(&store_ids, &row, "06", "SECOND_DISC_PER_STORE_06"),
            SECOND_DISC_PER_STORE_07: get_value(&store_ids, &row, "07", "SECOND_DISC_PER_STORE_07"),
            SECOND_DISC_PER_STORE_08: get_value(&store_ids, &row, "08", "SECOND_DISC_PER_STORE_08"),
            SECOND_DISC_PER_STORE_09: get_value(&store_ids, &row, "09", "SECOND_DISC_PER_STORE_09"),
            SECOND_DISC_PER_STORE_31: get_value(&store_ids, &row, "31", "SECOND_DISC_PER_STORE_31"),
            SECOND_DISC_PER_STORE_32: get_value(&store_ids, &row, "32", "SECOND_DISC_PER_STORE_32"),
            SECOND_DISC_PER_STORE_33: get_value(&store_ids, &row, "33", "SECOND_DISC_PER_STORE_33"),
            SECOND_DISC_PER_STORE_34: get_value(&store_ids, &row, "34", "SECOND_DISC_PER_STORE_34"),
            SECOND_DISC_PER_STORE_35: get_value(&store_ids, &row, "35", "SECOND_DISC_PER_STORE_35"),
            T_AVE_COST: if show_cost {
                row.get("T_AVE_COST").ok()
            } else {
                None
            },
        });
    }

    info!("Products Count: {:?}", products.len());

    Ok(products)
}
