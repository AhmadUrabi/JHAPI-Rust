use rocket::serde::Deserialize;
use rocket::serde::Serialize;

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

#[derive(serde::Deserialize, Debug, Serialize, Clone, PartialEq)]
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
