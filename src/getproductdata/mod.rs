use oracle::{Result};
use oracle::pool::Pool;
use rocket::serde::json::Json;

use std::time::UNIX_EPOCH;
use std::time::{SystemTime};

use crate::apistructs::Product;
use crate::apistructs::FetchParams;

pub async fn get_product_data(params: Json<FetchParams>, pool: &Pool) -> Option<Json<Vec<Product>>> {
    let s = get_product(params, pool).unwrap();
    if s.is_empty() {
        None
    } else {
        println!("{}", s.len());
        Some(Json(s.into_iter().map(|i| Product {
        ITEM_ID : i.ITEM_ID.clone(),
        IS_ACTIVE : i.IS_ACTIVE.clone(),
        CAN_BE_SOLD : i.CAN_BE_SOLD.clone(),
        ITEM_DESC : i.ITEM_DESC.clone(),
        ITEM_DESC_S : i.ITEM_DESC_S.clone(),
        FOREIGN_ITEM_CODE : i.FOREIGN_ITEM_CODE.clone(),
        ITEM_CAT : i.ITEM_CAT.clone(),
        ITEM_SUB_CAT : i.ITEM_SUB_CAT.clone(),
        SALE_UNIT : i.SALE_UNIT.clone(),
        UNIT_DESC : i.UNIT_DESC.clone(),
        PACKING : i.PACKING.clone(),
        CARD_OPEN_DATE : i.CARD_OPEN_DATE.clone(),
        HS_CODE : i.HS_CODE.clone(),
        COUNTRY : i.COUNTRY.clone(),
        COUNTRY_DESC : i.COUNTRY_DESC.clone(),
        SUPPLIER_ID : i.SUPPLIER_ID.clone(),
        SUPPLIER_DESC : i.SUPPLIER_DESC.clone(),
        ITEM_MAIN_BARCODE : i.ITEM_MAIN_BARCODE.clone(),
        NATURE_ID : i.NATURE_ID.clone(),
        NATURE_DESC : i.NATURE_DESC.clone(),
        TRADE_ID : i.TRADE_ID.clone(),
        TRADE_DESC : i.TRADE_DESC.clone(),
        QTY_STORE_01 : i.QTY_STORE_01.clone(),
        QTY_STORE_02 : i.QTY_STORE_02.clone(),
        QTY_STORE_05 : i.QTY_STORE_05.clone(),
        QTY_STORE_06 : i.QTY_STORE_06.clone(),
        QTY_STORE_07 : i.QTY_STORE_07.clone(),
        QTY_STORE_08 : i.QTY_STORE_08.clone(),
        QTY_STORE_10 : i.QTY_STORE_10.clone(),
        QTY_STORE_11 : i.QTY_STORE_11.clone(),
        QTY_STORE_12 : i.QTY_STORE_12.clone(),
        QTY_STORE_19 : i.QTY_STORE_19.clone(),
        QTY_STORE_21 : i.QTY_STORE_21.clone(),
        QTY_STORE_23 : i.QTY_STORE_23.clone(),
        QTY_STORE_31 : i.QTY_STORE_31.clone(),
        QTY_STORE_32 : i.QTY_STORE_32.clone(),
        QTY_STORE_33 : i.QTY_STORE_33.clone(),
        QTY_STORE_34 : i.QTY_STORE_34.clone(),
        QTY_STORE_35 : i.QTY_STORE_35.clone(),
        SALE_PRICE_NOTAX_STORE_01 : i.SALE_PRICE_NOTAX_STORE_01.clone(),
        SALE_PRICE_NOTAX_STORE_02 : i.SALE_PRICE_NOTAX_STORE_02.clone(),
        SALE_PRICE_NOTAX_STORE_05 : i.SALE_PRICE_NOTAX_STORE_05.clone(),
        SALE_PRICE_NOTAX_STORE_06 : i.SALE_PRICE_NOTAX_STORE_06.clone(),
        SALE_PRICE_NOTAX_STORE_08 : i.SALE_PRICE_NOTAX_STORE_08.clone(),
        SALE_PRICE_NOTAX_STORE_07 : i.SALE_PRICE_NOTAX_STORE_07.clone(),
        SALE_PRICE_NOTAX_STORE_31 : i.SALE_PRICE_NOTAX_STORE_31.clone(),
        SALE_PRICE_NOTAX_STORE_32 : i.SALE_PRICE_NOTAX_STORE_32.clone(),
        SALE_PRICE_NOTAX_STORE_33 : i.SALE_PRICE_NOTAX_STORE_33.clone(),
        SALE_PRICE_NOTAX_STORE_34 : i.SALE_PRICE_NOTAX_STORE_34.clone(),
        SALE_PRICE_NOTAX_STORE_35 : i.SALE_PRICE_NOTAX_STORE_35.clone(),
        FIRST_DISC_PER_STORE_01 : i.FIRST_DISC_PER_STORE_01.clone(),
        FIRST_DISC_PER_STORE_02 : i.FIRST_DISC_PER_STORE_02.clone(),
        FIRST_DISC_PER_STORE_05 : i.FIRST_DISC_PER_STORE_05.clone(),
        FIRST_DISC_PER_STORE_06 : i.FIRST_DISC_PER_STORE_06.clone(),
        FIRST_DISC_PER_STORE_07 : i.FIRST_DISC_PER_STORE_07.clone(),
        FIRST_DISC_PER_STORE_08 : i.FIRST_DISC_PER_STORE_08.clone(),
        FIRST_DISC_PER_STORE_31 : i.FIRST_DISC_PER_STORE_31.clone(),
        FIRST_DISC_PER_STORE_32 : i.FIRST_DISC_PER_STORE_32.clone(),
        FIRST_DISC_PER_STORE_33 : i.FIRST_DISC_PER_STORE_33.clone(),
        FIRST_DISC_PER_STORE_34 : i.FIRST_DISC_PER_STORE_34.clone(),
        FIRST_DISC_PER_STORE_35 : i.FIRST_DISC_PER_STORE_35.clone(),
        SECOND_DISC_PER_STORE_01 : i.SECOND_DISC_PER_STORE_01.clone(),
        SECOND_DISC_PER_STORE_02 : i.SECOND_DISC_PER_STORE_02.clone(),
        SECOND_DISC_PER_STORE_05 : i.SECOND_DISC_PER_STORE_05.clone(),
        SECOND_DISC_PER_STORE_06 : i.SECOND_DISC_PER_STORE_06.clone(),
        SECOND_DISC_PER_STORE_07 : i.SECOND_DISC_PER_STORE_07.clone(),
        SECOND_DISC_PER_STORE_08 : i.SECOND_DISC_PER_STORE_08.clone(),
        SECOND_DISC_PER_STORE_31 : i.SECOND_DISC_PER_STORE_31.clone(),
        SECOND_DISC_PER_STORE_32 : i.SECOND_DISC_PER_STORE_32.clone(),
        SECOND_DISC_PER_STORE_33 : i.SECOND_DISC_PER_STORE_33.clone(),
        SECOND_DISC_PER_STORE_34 : i.SECOND_DISC_PER_STORE_34.clone(),
        SECOND_DISC_PER_STORE_35 : i.SECOND_DISC_PER_STORE_35.clone(),
    }).collect()))
}
}


fn get_product(params: Json<FetchParams>, pool: &Pool) -> Result<Vec<Product>> {
    
    println!("params: {:?}", params);
    println!("Time Started: {:?}", SystemTime::now().duration_since(UNIX_EPOCH));


***REMOVED***
***REMOVED***
***REMOVED***
    

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

    println!("Time before connection: {:?}", SystemTime::now().duration_since(UNIX_EPOCH));


    let conn = pool.get()?;

    // Using column names made it hang on request -- On linux machine
    /* ITEM_ID, IS_ACTIVE, CAN_BE_SOLD, ITEM_DESC, ITEM_DESC_S, FOREIGN_ITEM_CODE, ITEM_CAT, ITEM_SUB_CAT, SALE_UNIT, UNIT_DESC, PACKING, CARD_OPEN_DATE, HS_CODE, COUNTRY, COUNTRY_DESC, SUPPLIER_ID, SUPPLIER_DESC, ITEM_MAIN_BARCODE, NATURE_ID, NATURE_DESC, TRADE_ID, TRADE_DESC, QTY_STORE_01, QTY_STORE_02, QTY_STORE_05, QTY_STORE_06, QTY_STORE_07, QTY_STORE_08, QTY_STORE_10, QTY_STORE_11, QTY_STORE_12, QTY_STORE_19, QTY_STORE_21, QTY_STORE_23, QTY_STORE_31, QTY_STORE_32, QTY_STORE_33, QTY_STORE_34, QTY_STORE_35, SALE_PRICE_NOTAX_STORE_01, SALE_PRICE_NOTAX_STORE_02, SALE_PRICE_NOTAX_STORE_05, SALE_PRICE_NOTAX_STORE_06, SALE_PRICE_NOTAX_STORE_08, SALE_PRICE_NOTAX_STORE_07, SALE_PRICE_NOTAX_STORE_31, SALE_PRICE_NOTAX_STORE_32, SALE_PRICE_NOTAX_STORE_33, SALE_PRICE_NOTAX_STORE_34, SALE_PRICE_NOTAX_STORE_35, FIRST_DISC_PER_STORE_01, FIRST_DISC_PER_STORE_02, FIRST_DISC_PER_STORE_05, FIRST_DISC_PER_STORE_06, FIRST_DISC_PER_STORE_07, FIRST_DISC_PER_STORE_08, FIRST_DISC_PER_STORE_31, FIRST_DISC_PER_STORE_32, FIRST_DISC_PER_STORE_33, FIRST_DISC_PER_STORE_34, FIRST_DISC_PER_STORE_35, SECOND_DISC_PER_STORE_01, SECOND_DISC_PER_STORE_02, SECOND_DISC_PER_STORE_05, SECOND_DISC_PER_STORE_06, SECOND_DISC_PER_STORE_07, SECOND_DISC_PER_STORE_08, SECOND_DISC_PER_STORE_31, SECOND_DISC_PER_STORE_32, SECOND_DISC_PER_STORE_33, SECOND_DISC_PER_STORE_34, SECOND_DISC_PER_STORE_35 */
    
    let mut stmt = conn.statement("SELECT * FROM ODBC_JHC.JHC_INVDATA WHERE FOREIGN_ITEM_CODE LIKE :ref AND ITEM_MAIN_BARCODE LIKE :barcode AND ITEM_ID LIKE :id").build()?;
    let rows = stmt.query(&[&mypRef,&mypBarcode,&mypId])?;
    println!("Time after fetching: {:?}", SystemTime::now().duration_since(UNIX_EPOCH));
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
        };
        products.push(prod);
    }
    println!("Time when vector ready: {:?}", SystemTime::now().duration_since(UNIX_EPOCH));
    Ok(products)
}