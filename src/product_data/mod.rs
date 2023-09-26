use oracle::pool::Pool;
#[allow(non_snake_case)]
use oracle::Result;

use rocket::log::private::info;
use rocket::serde::json::Json;

use crate::product_data::structs::FetchParams;
use crate::product_data::structs::Product;

use crate::ApiKey;

use crate::utils::permissions::is_cost_perm;

pub mod structs;

pub async fn get_product(
    params: Json<FetchParams>,
    pool: &Pool,
    key: ApiKey<'_>,
) -> Result<Vec<Product>> {
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

    let mut stmt = conn.statement("SELECT ITEM_ID, IS_ACTIVE, CAN_BE_SOLD, ITEM_DESC, ITEM_DESC_S, FOREIGN_ITEM_CODE, ITEM_CAT, ITEM_SUB_CAT, SALE_UNIT, UNIT_DESC, PACKING, CARD_OPEN_DATE, HS_CODE, COUNTRY, COUNTRY_DESC, SUPPLIER_ID, SUPPLIER_DESC, ITEM_MAIN_BARCODE, NATURE_ID, NATURE_DESC, TRADE_ID, TRADE_DESC, QTY_STORE_01, QTY_STORE_02, QTY_STORE_05, QTY_STORE_06, QTY_STORE_07, QTY_STORE_08, QTY_STORE_09, QTY_STORE_10, QTY_STORE_11, QTY_STORE_12, QTY_STORE_19, QTY_STORE_21, QTY_STORE_23, QTY_STORE_31, QTY_STORE_32, QTY_STORE_33, QTY_STORE_34, QTY_STORE_35, SALE_PRICE_NOTAX_STORE_01, SALE_PRICE_NOTAX_STORE_02, SALE_PRICE_NOTAX_STORE_05, SALE_PRICE_NOTAX_STORE_06, SALE_PRICE_NOTAX_STORE_08, SALE_PRICE_NOTAX_STORE_09, SALE_PRICE_NOTAX_STORE_07, SALE_PRICE_NOTAX_STORE_31, SALE_PRICE_NOTAX_STORE_32, SALE_PRICE_NOTAX_STORE_33, SALE_PRICE_NOTAX_STORE_34, SALE_PRICE_NOTAX_STORE_35, FIRST_DISC_PER_STORE_01, FIRST_DISC_PER_STORE_02, FIRST_DISC_PER_STORE_05, FIRST_DISC_PER_STORE_06, FIRST_DISC_PER_STORE_07, FIRST_DISC_PER_STORE_08, FIRST_DISC_PER_STORE_09, FIRST_DISC_PER_STORE_31, FIRST_DISC_PER_STORE_32, FIRST_DISC_PER_STORE_33, FIRST_DISC_PER_STORE_34, FIRST_DISC_PER_STORE_35, SECOND_DISC_PER_STORE_01, SECOND_DISC_PER_STORE_02, SECOND_DISC_PER_STORE_05, SECOND_DISC_PER_STORE_06, SECOND_DISC_PER_STORE_07, SECOND_DISC_PER_STORE_08, SECOND_DISC_PER_STORE_09, SECOND_DISC_PER_STORE_31, SECOND_DISC_PER_STORE_32, SECOND_DISC_PER_STORE_33, SECOND_DISC_PER_STORE_34, SECOND_DISC_PER_STORE_35, T_AVE_COST FROM ODBC_JHC.JHC_INVDATA WHERE FOREIGN_ITEM_CODE LIKE :ref AND ITEM_MAIN_BARCODE LIKE :barcode AND ITEM_ID LIKE :id").build()?;
    let rows = stmt.query(&[&mypRef, &mypBarcode, &mypId])?;

    let mut products: Vec<Product> = vec![];

    for row_result in rows {
        let row = row_result?;
        products.push(Product {
            ITEM_ID: row.get(0)?,
            IS_ACTIVE: row.get(1)?,
            CAN_BE_SOLD: row.get(2)?,
            ITEM_DESC: row.get(3)?,
            ITEM_DESC_S: row.get(4)?,
            FOREIGN_ITEM_CODE: row.get(5)?,
            ITEM_CAT: row.get(6)?,
            ITEM_SUB_CAT: row.get(7)?,
            SALE_UNIT: row.get(8)?,
            UNIT_DESC: row.get(9)?,
            PACKING: row.get(10)?,
            CARD_OPEN_DATE: row.get(11)?,
            HS_CODE: row.get(12)?,
            COUNTRY: row.get(13)?,
            COUNTRY_DESC: row.get(14)?,
            SUPPLIER_ID: row.get(15)?,
            SUPPLIER_DESC: row.get(16)?,
            ITEM_MAIN_BARCODE: row.get(17)?,
            NATURE_ID: row.get(18)?,
            NATURE_DESC: row.get(19)?,
            TRADE_ID: row.get(20)?,
            TRADE_DESC: row.get(21)?,
            QTY_STORE_01: row.get(22)?,
            QTY_STORE_02: row.get(23)?,
            QTY_STORE_05: row.get(24)?,
            QTY_STORE_06: row.get(25)?,
            QTY_STORE_07: row.get(26)?,
            QTY_STORE_08: row.get(27)?,
            QTY_STORE_09: row.get(28)?,
            QTY_STORE_10: row.get(29)?,
            QTY_STORE_11: row.get(30)?,
            QTY_STORE_12: row.get(31)?,
            QTY_STORE_19: row.get(32)?,
            QTY_STORE_21: row.get(33)?,
            QTY_STORE_23: row.get(34)?,
            QTY_STORE_31: row.get(35)?,
            QTY_STORE_32: row.get(36)?,
            QTY_STORE_33: row.get(37)?,
            QTY_STORE_34: row.get(38)?,
            QTY_STORE_35: row.get(39)?,
            SALE_PRICE_NOTAX_STORE_01: row.get(40)?,
            SALE_PRICE_NOTAX_STORE_02: row.get(41)?,
            SALE_PRICE_NOTAX_STORE_05: row.get(42)?,
            SALE_PRICE_NOTAX_STORE_06: row.get(43)?,
            SALE_PRICE_NOTAX_STORE_08: row.get(44)?,
            SALE_PRICE_NOTAX_STORE_09: row.get(45)?,
            SALE_PRICE_NOTAX_STORE_07: row.get(46)?,
            SALE_PRICE_NOTAX_STORE_31: row.get(47)?,
            SALE_PRICE_NOTAX_STORE_32: row.get(48)?,
            SALE_PRICE_NOTAX_STORE_33: row.get(49)?,
            SALE_PRICE_NOTAX_STORE_34: row.get(50)?,
            SALE_PRICE_NOTAX_STORE_35: row.get(51)?,
            FIRST_DISC_PER_STORE_01: row.get(52)?,
            FIRST_DISC_PER_STORE_02: row.get(53)?,
            FIRST_DISC_PER_STORE_05: row.get(54)?,
            FIRST_DISC_PER_STORE_06: row.get(55)?,
            FIRST_DISC_PER_STORE_07: row.get(56)?,
            FIRST_DISC_PER_STORE_08: row.get(57)?,
            FIRST_DISC_PER_STORE_09: row.get(58)?,
            FIRST_DISC_PER_STORE_31: row.get(59)?,
            FIRST_DISC_PER_STORE_32: row.get(60)?,
            FIRST_DISC_PER_STORE_33: row.get(61)?,
            FIRST_DISC_PER_STORE_34: row.get(62)?,
            FIRST_DISC_PER_STORE_35: row.get(63)?,
            SECOND_DISC_PER_STORE_01: row.get(64)?,
            SECOND_DISC_PER_STORE_02: row.get(65)?,
            SECOND_DISC_PER_STORE_05: row.get(66)?,
            SECOND_DISC_PER_STORE_06: row.get(67)?,
            SECOND_DISC_PER_STORE_07: row.get(68)?,
            SECOND_DISC_PER_STORE_08: row.get(69)?,
            SECOND_DISC_PER_STORE_09: row.get(70)?,
            SECOND_DISC_PER_STORE_31: row.get(71)?,
            SECOND_DISC_PER_STORE_32: row.get(72)?,
            SECOND_DISC_PER_STORE_33: row.get(73)?,
            SECOND_DISC_PER_STORE_34: row.get(74)?,
            SECOND_DISC_PER_STORE_35: row.get(75)?,
            T_AVE_COST: if is_cost_perm(&key, pool) {
                row.get(76)?
            } else {
                None
            },
        });
    }

    info!("Products Count: {:?}", products.len());

    Ok(products)
}
