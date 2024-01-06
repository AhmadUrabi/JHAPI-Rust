#[allow(non_snake_case)]
use std::collections::HashSet;

use oracle::pool::Pool;
use oracle::Row;

use oracle::sql_type::ToSql;
use rocket::log::private::info;
use rocket::serde::json::Json;

use crate::product_data::structs::FetchParams;
use crate::product_data::structs::Product;

use crate::fetch_stores::get_stores;

use crate::ApiKey;

use crate::signing::decode_token_data;
use crate::utils::permissions::is_cost_perm;
use crate::utils::structs::APIErrors;

pub mod structs;

// TODO: Test performance of this function
pub async fn get_product(
    params: Json<FetchParams>,
    pool: &Pool,
    key: &ApiKey<'_>,
) -> Result<Vec<Product>, APIErrors> {
    // Empty params are not an error, but they should return an empty vec
    if params.p_ref.is_none() && params.p_barcode.is_none() && params.p_id.is_none() {
        // vec![] is shorthand for Vec::new(), omitted for compile time optimization
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
            return Err(APIErrors::InvalidToken);
        }
    }

    // To ensure user only gets data for stores they have access to
    // This does have a performance penalty, as the function has to touch the DB twice
    // Could be optimized by creating a more complex query, or by using hashsets
    // TODO Test using a more complex query
    let mut store_ids: HashSet<String> = HashSet::new();

    match get_stores(pool, username) {
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
    let show_cost = is_cost_perm(key, pool);

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

    let mut sql = String::from("SELECT * FROM ODBC_JHC.JHC_INVDATA");
    let mut my_params: Vec<(&str, &dyn ToSql)> = Vec::new();

    if let Some(p_ref) = &params.p_ref {
        if p_ref.contains("%") {
            sql.push_str(" WHERE FOREIGN_ITEM_CODE LIKE :ref");
        } else {
            sql.push_str(" WHERE FOREIGN_ITEM_CODE = :ref");
        }
        my_params.push(("ref", p_ref as &dyn ToSql));
    }

    if let Some(p_barcode) = &params.p_barcode {
        if p_barcode.contains("%") {
            sql.push_str(" AND ITEM_MAIN_BARCODE LIKE :barcode");
        } else {
            sql.push_str(" AND ITEM_MAIN_BARCODE = :barcode");
        }
        my_params.push(("barcode", p_barcode as &dyn ToSql));
    }

    if let Some(p_id) = &params.p_id {
        if p_id.contains("%") {
            sql.push_str(" AND ITEM_ID LIKE :id");
        } else {
            sql.push_str(" AND ITEM_ID = :id");
        }
        my_params.push(("id", p_id as &dyn ToSql));
    }

    if my_params.is_empty() {
        return Ok(Vec::new());
    }


    let conn = pool.get();
    if conn.is_err() {
        info!("Connection Error");
        return Err(APIErrors::DBError);
    }
    let conn = conn.unwrap();

    // TODO: Fix error handling on the query

    let mut stmt;
    let rows;
    let now = tokio::time::Instant::now();

    stmt = conn.statement(&sql).build().unwrap();
    rows = stmt.query_named(&my_params).unwrap();

    
    println!("Total Query Time: {:?}", now.elapsed().as_millis());


    let mut products: Vec<Product> = Vec::new();

    for row_result in rows {
        if row_result.is_err() {
            info!("Row Error");
            return Err(APIErrors::DBError);
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
