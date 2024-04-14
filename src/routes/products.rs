#![allow(non_snake_case)]
use std::path::PathBuf;
use std::str::FromStr;

use crate::functions::files::get_file_stream;
use crate::get_image;
use crate::server::JHApiServerState;
use crate::utils::permissions::{has_admin_perm, has_reports_perm};

use genpdf::{elements, style, Element};
use rocket::fs::NamedFile;
use rocket::futures::stream;
use rocket::http::Status;
use rocket::log::private::info;
use rocket::serde::json::Json;
use rocket::{post, State};

use crate::functions::products::{get_product, get_product_by_supplier};
use crate::server::request_guard::api_key::ApiKey;

use crate::functions::products::structs::FetchParams;
use crate::functions::products::structs::Product;

#[post("/products", data = "<params>")]
pub async fn get_products(
    params: Json<FetchParams>,
    state: &State<JHApiServerState>,
    key: ApiKey<'_>,
) -> Json<Vec<Product>> {
    let pool = &state.pool;
    let sql_manager = &state.sql_manager;
    info!("GetProductData Request: {:?}", params);
    match get_product(params, &pool, &sql_manager, &key).await {
        Ok(products) => Json(products),
        Err(_err) => {
            error!("Error");
            Json(vec![])
        }
    }
}

#[get("/products/supplies/<supplier_id>")]
pub async fn get_products_by_supplier(
    state: &State<JHApiServerState>,
    key: ApiKey<'_>,
    supplier_id: i32,
) -> Result<Json<Vec<Product>>, Status> {
    let pool = &state.pool;
    let sql_manager = &state.sql_manager;
    info!("GetProductData Request: {:?}", supplier_id);

    // if !has_reports_perm(&key, &pool, &sql_manager).await && !has_admin_perm(&key, &pool, &sql_manager).await {
    // return Err(Status::Unauthorized);
    // }

    match get_product_by_supplier(supplier_id, &pool, &sql_manager, &key).await {
        Ok(products) => Ok(Json(products)),
        Err(_err) => {
            error!("Error");
            Err(Status::InternalServerError)
        }
    }
}

#[get("/reports")]
pub async fn print_test_report(
    state: &State<JHApiServerState>,
    key: ApiKey<'_>,
) -> Result<Option<NamedFile>, Status> {
    let pool = &state.pool;
    let sql_manager = &state.sql_manager;
    let products = get_product_by_supplier(200, &pool, &sql_manager, &key)
        .await
        .unwrap();

    // tokio spawn thread
    let handle = tokio::spawn(async move {
        //print current path
        println!("{:?}", std::env::current_dir().unwrap());
        let path = std::path::Path::new("./fonts/");
        let font_family =
            genpdf::fonts::from_files(path, "Roboto", None).unwrap();
        // Create a document and set the default font family
        let mut doc = genpdf::Document::new(font_family);
        // Change the default settings
        doc.set_title("Contigo Store 2 Report");
        // Customize the pages
        let mut decorator = genpdf::SimplePageDecorator::new();
        decorator.set_margins(10);
        doc.set_page_decorator(decorator);
        // Add one or more elements

        // Add a table
        let mut table = genpdf::elements::TableLayout::new(vec![2, 1, 4 ,1,1]);
        table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));
        table
        .row()
        .element(
            elements::Paragraph::new("Image")
                .styled(style::Effect::Bold)
                .padded(1),
        )
        .element(
            elements::Paragraph::new("Product ID")
                .styled(style::Effect::Bold)
                .padded(1),
        )
        .element(
            elements::Paragraph::new("Product Desc")
                .styled(style::Effect::Bold)
                .padded(1),
        )
        .element(
            elements::Paragraph::new("Store 2 price")
                .styled(style::Effect::Bold)
                .padded(1),
        )
        .element(
            elements::Paragraph::new("Store 2 QTY")
                .styled(style::Effect::Bold)
                .padded(1),
        )
        .push()
        .expect("Invalid table row");

        for product in products {
            // Spawn another thread 
            let temp = product.ITEM_ID.clone().unwrap();
            let stream = get_file_stream(temp.as_str());
            
            if stream.is_err() {
                println!("Error: {:?}", stream.err().unwrap());
                continue;
            }
            let image = genpdf::elements::Image::from_reader(stream.unwrap()).unwrap();

            table
                .row()
                .element(
                    image.padded(1)
                )
                .element(elements::Paragraph::new(product.ITEM_ID.unwrap()).padded(1))
                .element(elements::Paragraph::new(product.ITEM_DESC_S.unwrap()).padded(1))
                .element(elements::Paragraph::new(product.SALE_PRICE_NOTAX_STORE_02.unwrap()).padded(1))
                .element(elements::Paragraph::new(product.QTY_STORE_02.unwrap()).padded(1))
                .push()
                .expect("Invalid table row");
        }
        doc.push(table);


        // Render the document and write it to a file
        doc.render_to_file("tmp/output.pdf")
            .expect("Failed to write PDF file");
    });
    let _ = handle.await;
    Ok(NamedFile::open("tmp/output.pdf").await.ok())
}

/*
#[post("/GetProductDataPI", data = "<params>")]
pub async fn get_products_pi(
    params: Json<FetchParams>,
    state: &State<JHApiServerState>,
    _key: ApiKey<'_>,
    client_ip: Option<IpAddr>,
    log_check: LogCheck,
) -> Json<Vec<Product>> {
    info!("GetProductData Request: {:?}", params);
    info!("Client IP: {:?}", client_ip);
    #[allow(unused_assignments)]
    let mut username: String = "".to_string();
    match decode_token_data(_key.0) {
        Some(data) => {
            info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap());
            username = data.USER_ID.unwrap();
        }
        None => {
            info!("Token Data: None");
            username = "None".to_string();
        }
    }

    let tokenUsed = _key.0.to_string();

    // Convert json to String

    let params_clone = params.clone();

    match get_product_pi(params, pool, &_key).await {
        Ok(products) => {
            if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
            log_data(
                pool,
                username,
                client_ip.unwrap().to_string(),
                "/GetProductData".to_string(),
                Some(serde_json::to_string(&params_clone.0).unwrap()),
                get_timestamp(),
                tokenUsed,
                "Success".to_string(),
                "GET".to_string()
            );
        }
            Json(products)
        }
        Err(err) => {
            error!("Error: {}", err);
            if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
            log_data(
                pool,
                username,
                client_ip.unwrap().to_string(),
                "/GetProductData".to_string(),
                Some(serde_json::to_string(&params_clone.0).unwrap()),
                get_timestamp(),
                tokenUsed,
                "Error Fetching".to_string(),
                "GET".to_string()
            );
        }
            Json(vec![])
        }
    }
}
*/
