#[allow(non_snake_case)]
use oracle::Result;
use oracle::pool::Pool;

use rocket::log::private::info;
use rocket::serde::json::Json;

use crate::product_data::structs::Product;
use crate::product_data::structs::FetchParams;

use crate::ApiKey;

use crate::utils::permissions::is_cost_perm;

pub mod structs;

pub fn get_product(params: Json<FetchParams>, pool: &Pool, key : ApiKey<'_>) -> Result<Vec<Product>> {
    if params.pRef.is_none() && params.pBarcode.is_none() && params.pId.is_none() {
        return Ok(vec![]);
    }

    let mut mypRef = "%";
    let mut mypBarcode = "%";
    let mut mypId = "%";


    if let Some(pRef) = &params.pRef {
        mypRef = pRef;
    }

    if let Some(pBarcode) = &params.pBarcode {
        mypBarcode = pBarcode;
    }

    if let Some(pId) = &params.pId {
        mypId = pId;
    }

    let conn = pool.get()?;

    // Using column names made it hang on request -- On linux machine
    /* ITEM_ID, IS_ACTIVE, CAN_BE_SOLD, ITEM_DESC, ITEM_DESC_S, FOREIGN_ITEM_CODE, ITEM_CAT, ITEM_SUB_CAT, SALE_UNIT, UNIT_DESC, PACKING, CARD_OPEN_DATE, HS_CODE, COUNTRY, COUNTRY_DESC, SUPPLIER_ID, SUPPLIER_DESC, ITEM_MAIN_BARCODE, NATURE_ID, NATURE_DESC, TRADE_ID, TRADE_DESC, QTY_STORE_01, QTY_STORE_02, QTY_STORE_05, QTY_STORE_06, QTY_STORE_07, QTY_STORE_08, QTY_STORE_10, QTY_STORE_11, QTY_STORE_12, QTY_STORE_19, QTY_STORE_21, QTY_STORE_23, QTY_STORE_31, QTY_STORE_32, QTY_STORE_33, QTY_STORE_34, QTY_STORE_35, SALE_PRICE_NOTAX_STORE_01, SALE_PRICE_NOTAX_STORE_02, SALE_PRICE_NOTAX_STORE_05, SALE_PRICE_NOTAX_STORE_06, SALE_PRICE_NOTAX_STORE_08, SALE_PRICE_NOTAX_STORE_07, SALE_PRICE_NOTAX_STORE_31, SALE_PRICE_NOTAX_STORE_32, SALE_PRICE_NOTAX_STORE_33, SALE_PRICE_NOTAX_STORE_34, SALE_PRICE_NOTAX_STORE_35, FIRST_DISC_PER_STORE_01, FIRST_DISC_PER_STORE_02, FIRST_DISC_PER_STORE_05, FIRST_DISC_PER_STORE_06, FIRST_DISC_PER_STORE_07, FIRST_DISC_PER_STORE_08, FIRST_DISC_PER_STORE_31, FIRST_DISC_PER_STORE_32, FIRST_DISC_PER_STORE_33, FIRST_DISC_PER_STORE_34, FIRST_DISC_PER_STORE_35, SECOND_DISC_PER_STORE_01, SECOND_DISC_PER_STORE_02, SECOND_DISC_PER_STORE_05, SECOND_DISC_PER_STORE_06, SECOND_DISC_PER_STORE_07, SECOND_DISC_PER_STORE_08, SECOND_DISC_PER_STORE_31, SECOND_DISC_PER_STORE_32, SECOND_DISC_PER_STORE_33, SECOND_DISC_PER_STORE_34, SECOND_DISC_PER_STORE_35 */
    
    let mut stmt = conn.statement("SELECT * FROM ODBC_JHC.JHC_INVDATA WHERE FOREIGN_ITEM_CODE LIKE :ref AND ITEM_MAIN_BARCODE LIKE :barcode AND ITEM_ID LIKE :id").build()?;
    let rows = stmt.query(&[&mypRef,&mypBarcode,&mypId])?;

    let mut products : Vec<Product> = vec![];
    
    for row_result in rows {
        let row = row_result?;

        let prod = Product {
            ITEM_ID : row.get("ITEM_ID")?,
            IS_ACTIVE : row.get("IS_ACTIVE")?,
            CAN_BE_SOLD : row.get("CAN_BE_SOLD")?,
            ITEM_DESC : row.get("ITEM_DESC")?,
            ITEM_DESC_S : row.get("ITEM_DESC_S")?,
            FOREIGN_ITEM_CODE : row.get("FOREIGN_ITEM_CODE")?,
            ITEM_CAT : row.get("ITEM_CAT")?,
            ITEM_SUB_CAT : row.get("ITEM_SUB_CAT")?,
            SALE_UNIT : row.get("SALE_UNIT")?,
            UNIT_DESC : row.get("UNIT_DESC")?,
            PACKING : row.get("PACKING")?,
            CARD_OPEN_DATE : row.get("CARD_OPEN_DATE")?,
            HS_CODE : row.get("HS_CODE")?,
            COUNTRY : row.get("COUNTRY")?,
            COUNTRY_DESC : row.get("COUNTRY_DESC")?,
            SUPPLIER_ID : row.get("SUPPLIER_ID")?,
            SUPPLIER_DESC : row.get("SUPPLIER_DESC")?,
            ITEM_MAIN_BARCODE : row.get("ITEM_MAIN_BARCODE")?,
            NATURE_ID : row.get("NATURE_ID")?,
            NATURE_DESC : row.get("NATURE_DESC")?,
            TRADE_ID : row.get("TRADE_ID")?,
            TRADE_DESC : row.get("TRADE_DESC")?,
            QTY_STORE_01 : row.get("QTY_STORE_01")?,
            QTY_STORE_02 : row.get("QTY_STORE_02")?,
            QTY_STORE_05 : row.get("QTY_STORE_05")?,
            QTY_STORE_06 : row.get("QTY_STORE_06")?,
            QTY_STORE_07 : row.get("QTY_STORE_07")?,
            QTY_STORE_08 : row.get("QTY_STORE_08")?,
            QTY_STORE_10 : row.get("QTY_STORE_10")?,
            QTY_STORE_11 : row.get("QTY_STORE_11")?,
            QTY_STORE_12 : row.get("QTY_STORE_12")?,
            QTY_STORE_19 : row.get("QTY_STORE_19")?,
            QTY_STORE_21 : row.get("QTY_STORE_21")?,
            QTY_STORE_23 : row.get("QTY_STORE_23")?,
            QTY_STORE_31 : row.get("QTY_STORE_31")?,
            QTY_STORE_32 : row.get("QTY_STORE_32")?,
            QTY_STORE_33 : row.get("QTY_STORE_33")?,
            QTY_STORE_34 : row.get("QTY_STORE_34")?,
            QTY_STORE_35 : row.get("QTY_STORE_35")?,
            SALE_PRICE_NOTAX_STORE_01 : row.get("SALE_PRICE_NOTAX_STORE_01")?, 
            SALE_PRICE_NOTAX_STORE_02 : row.get("SALE_PRICE_NOTAX_STORE_02")?, 
            SALE_PRICE_NOTAX_STORE_05 : row.get("SALE_PRICE_NOTAX_STORE_05")?, 
            SALE_PRICE_NOTAX_STORE_06 : row.get("SALE_PRICE_NOTAX_STORE_06")?, 
            SALE_PRICE_NOTAX_STORE_08 : row.get("SALE_PRICE_NOTAX_STORE_08")?, 
            SALE_PRICE_NOTAX_STORE_07 : row.get("SALE_PRICE_NOTAX_STORE_07")?, 
            SALE_PRICE_NOTAX_STORE_31 : row.get("SALE_PRICE_NOTAX_STORE_31")?, 
            SALE_PRICE_NOTAX_STORE_32 : row.get("SALE_PRICE_NOTAX_STORE_32")?, 
            SALE_PRICE_NOTAX_STORE_33 : row.get("SALE_PRICE_NOTAX_STORE_33")?, 
            SALE_PRICE_NOTAX_STORE_34 : row.get("SALE_PRICE_NOTAX_STORE_34")?, 
            SALE_PRICE_NOTAX_STORE_35 : row.get("SALE_PRICE_NOTAX_STORE_35")?, 
            FIRST_DISC_PER_STORE_01 : row.get("FIRST_DISC_PER_STORE_01")?,
            FIRST_DISC_PER_STORE_02 : row.get("FIRST_DISC_PER_STORE_02")?,
            FIRST_DISC_PER_STORE_05 : row.get("FIRST_DISC_PER_STORE_05")?,
            FIRST_DISC_PER_STORE_06 : row.get("FIRST_DISC_PER_STORE_06")?,
            FIRST_DISC_PER_STORE_07 : row.get("FIRST_DISC_PER_STORE_07")?,
            FIRST_DISC_PER_STORE_08 : row.get("FIRST_DISC_PER_STORE_08")?,
            FIRST_DISC_PER_STORE_31 : row.get("FIRST_DISC_PER_STORE_31")?,
            FIRST_DISC_PER_STORE_32 : row.get("FIRST_DISC_PER_STORE_32")?,
            FIRST_DISC_PER_STORE_33 : row.get("FIRST_DISC_PER_STORE_33")?,
            FIRST_DISC_PER_STORE_34 : row.get("FIRST_DISC_PER_STORE_34")?,
            FIRST_DISC_PER_STORE_35 : row.get("FIRST_DISC_PER_STORE_35")?,
            SECOND_DISC_PER_STORE_01 : row.get("SECOND_DISC_PER_STORE_01")?,
            SECOND_DISC_PER_STORE_02 : row.get("SECOND_DISC_PER_STORE_02")?,
            SECOND_DISC_PER_STORE_05 : row.get("SECOND_DISC_PER_STORE_05")?,
            SECOND_DISC_PER_STORE_06 : row.get("SECOND_DISC_PER_STORE_06")?,
            SECOND_DISC_PER_STORE_07 : row.get("SECOND_DISC_PER_STORE_07")?,
            SECOND_DISC_PER_STORE_08 : row.get("SECOND_DISC_PER_STORE_08")?,
            SECOND_DISC_PER_STORE_31 : row.get("SECOND_DISC_PER_STORE_31")?,
            SECOND_DISC_PER_STORE_32 : row.get("SECOND_DISC_PER_STORE_32")?,
            SECOND_DISC_PER_STORE_33 : row.get("SECOND_DISC_PER_STORE_33")?,
            SECOND_DISC_PER_STORE_34 : row.get("SECOND_DISC_PER_STORE_34")?,
            SECOND_DISC_PER_STORE_35 : row.get("SECOND_DISC_PER_STORE_35")?,
            T_AVE_COST : if is_cost_perm(&key, pool) {row.get("T_AVE_COST")?} else {None},
        };
        products.push(prod);
    }

    info!("Products Count: {:?}", products.len());

    Ok(products)
}