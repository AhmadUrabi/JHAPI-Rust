use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Product {
    pub ITEM_ID : Option<String>,
    pub IS_ACTIVE : Option<String>,
    pub CAN_BE_SOLD : Option<String>,
    pub ITEM_DESC : Option<String>,
    pub ITEM_DESC_S : Option<String>,
    pub FOREIGN_ITEM_CODE : Option<String>,
    pub ITEM_CAT : Option<String>,
    pub ITEM_SUB_CAT : Option<String>,
    pub SALE_UNIT : Option<String>,
    pub UNIT_DESC : Option<String>,
    pub PACKING : Option<String>,
    pub CARD_OPEN_DATE : Option<String>,
    pub HS_CODE : Option<String>,
    pub COUNTRY : Option<String>,
    pub COUNTRY_DESC : Option<String>,
    pub SUPPLIER_ID : Option<String>,
    pub SUPPLIER_DESC : Option<String>,
    pub ITEM_MAIN_BARCODE : Option<String>,
    pub NATURE_ID : Option<String>,
    pub NATURE_DESC : Option<String>,
    pub TRADE_ID : Option<String>,
    pub TRADE_DESC : Option<String>,
    pub QTY_STORE_01 : Option<String>,
    pub QTY_STORE_02 : Option<String>,
    pub QTY_STORE_05 : Option<String>,
    pub QTY_STORE_06 : Option<String>,
    pub QTY_STORE_07 : Option<String>,
    pub QTY_STORE_08 : Option<String>,
    pub QTY_STORE_10 : Option<String>,
    pub QTY_STORE_11 : Option<String>,
    pub QTY_STORE_12 : Option<String>,
    pub QTY_STORE_19 : Option<String>,
    pub QTY_STORE_21 : Option<String>,
    pub QTY_STORE_23 : Option<String>,
    pub QTY_STORE_31 : Option<String>,
    pub QTY_STORE_32 : Option<String>,
    pub QTY_STORE_33 : Option<String>,
    pub QTY_STORE_34 : Option<String>,
    pub QTY_STORE_35 : Option<String>,
    pub SALE_PRICE_NOTAX_STORE_01 : Option<String>,
    pub SALE_PRICE_NOTAX_STORE_02 : Option<String>,
    pub SALE_PRICE_NOTAX_STORE_05 : Option<String>,
    pub SALE_PRICE_NOTAX_STORE_06 : Option<String>,
    pub SALE_PRICE_NOTAX_STORE_08 : Option<String>,
    pub SALE_PRICE_NOTAX_STORE_07 : Option<String>,
    pub SALE_PRICE_NOTAX_STORE_31 : Option<String>,
    pub SALE_PRICE_NOTAX_STORE_32 : Option<String>,
    pub SALE_PRICE_NOTAX_STORE_33 : Option<String>,
    pub SALE_PRICE_NOTAX_STORE_34 : Option<String>,
    pub SALE_PRICE_NOTAX_STORE_35 : Option<String>,
    pub FIRST_DISC_PER_STORE_01 : Option<String>,
    pub FIRST_DISC_PER_STORE_02 : Option<String>,
    pub FIRST_DISC_PER_STORE_05 : Option<String>,
    pub FIRST_DISC_PER_STORE_06 : Option<String>,
    pub FIRST_DISC_PER_STORE_07 : Option<String>,
    pub FIRST_DISC_PER_STORE_08 : Option<String>,
    pub FIRST_DISC_PER_STORE_31 : Option<String>,
    pub FIRST_DISC_PER_STORE_32 : Option<String>,
    pub FIRST_DISC_PER_STORE_33 : Option<String>,
    pub FIRST_DISC_PER_STORE_34 : Option<String>,
    pub FIRST_DISC_PER_STORE_35 : Option<String>,
    pub SECOND_DISC_PER_STORE_01 : Option<String>,
    pub SECOND_DISC_PER_STORE_02 : Option<String>,
    pub SECOND_DISC_PER_STORE_05 : Option<String>,
    pub SECOND_DISC_PER_STORE_06 : Option<String>,
    pub SECOND_DISC_PER_STORE_07 : Option<String>,
    pub SECOND_DISC_PER_STORE_08 : Option<String>,
    pub SECOND_DISC_PER_STORE_31 : Option<String>,
    pub SECOND_DISC_PER_STORE_32 : Option<String>,
    pub SECOND_DISC_PER_STORE_33 : Option<String>,
    pub SECOND_DISC_PER_STORE_34 : Option<String>,
    pub SECOND_DISC_PER_STORE_35 : Option<String>,
    
}

#[derive(serde::Deserialize, Debug)]
pub struct FetchParams {
    pub pRef: Option<String>,
    pub pBarcode: Option<String>,
    pub pId: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct Store {
    pub STORE_ID : Option<String>,
    pub STORE_DESC : Option<String>,
    pub STORE_DESC_S : Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub USER_ID : Option<String>,
    pub USER_NAME : Option<String>,
    pub USER_EMAIL : Option<String>,
    pub LOGIN_DURATION : Option<String>
}

#[derive(serde::Deserialize, Debug)]
pub struct LoginParams {
    pub pUserName: Option<String>,
    pub pPassword: Option<String>
}