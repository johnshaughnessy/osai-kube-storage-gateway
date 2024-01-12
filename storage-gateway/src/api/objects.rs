use actix_web::{web, HttpResponse, Responder};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use server::models::{CreateObjectPayload, UpdateObjectPayload, ID};

pub async fn post_objects(
    create_object_payload: web::Json<CreateObjectPayload>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let create_object_payload = create_object_payload.into_inner();
    match server::models::objects::create_object(&mut conn, &create_object_payload) {
        Ok(object) => HttpResponse::Ok().body(format!("Object added: {:?}", object)),
        Err(_) => HttpResponse::InternalServerError().body("Error adding object."),
    }
}

pub async fn get_objects(pool: web::Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    log::info!("get_objects");
    let mut conn = pool.get().unwrap();

    match server::models::objects::read_objects(&mut conn) {
        Ok(objects) => HttpResponse::Ok().json(objects),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error retrieving objects: {:?}", e))
        } //        Err(_) => HttpResponse::InternalServerError().body("Error retrieving objects."),
    }
}

pub async fn patch_objects(
    path: web::Path<(ID,)>,
    update_object_payload: web::Json<UpdateObjectPayload>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let target_object_id: ID = path.0;
    let update_object_payload = update_object_payload.into_inner();

    match server::models::objects::update_object(
        &mut conn,
        target_object_id,
        &update_object_payload,
    ) {
        Ok(objects) => HttpResponse::Ok().json(objects),
        Err(_) => HttpResponse::InternalServerError().body("Error retrieving objects."),
    }
}

pub async fn delete_objects(
    path: web::Path<(ID,)>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let target_object_id: ID = path.0;

    match server::models::objects::delete_object(&mut conn, target_object_id) {
        Ok(objects) => HttpResponse::Ok().json(objects),
        Err(_) => HttpResponse::InternalServerError().body("Error retrieving objects."),
    }
}
