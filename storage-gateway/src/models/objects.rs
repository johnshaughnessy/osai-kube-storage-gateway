use chrono::NaiveDateTime;
use diesel::deserialize::QueryableByName;
use diesel::prelude::*;
use diesel::update;
use diesel::Insertable;
use log::info;
use serde_derive::{Deserialize, Serialize};

use crate::schema::objects;

use super::ID;

#[derive(
    Debug, Copy, Clone, Serialize, Deserialize, Queryable, QueryableByName, Insertable, Identifiable,
)]
#[diesel(table_name = crate::schema::objects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(object_id))]
pub struct Object {
    pub object_id: ID,
    #[serde(skip_serializing)]
    pub inserted_at: NaiveDateTime,
    #[serde(skip_serializing)]
    pub updated_at: NaiveDateTime,
    pub measured_at: NaiveDateTime,
    pub object_kg: f64,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::objects)]
pub struct CreateObjectPayload {
    pub measured_at: NaiveDateTime,
    pub object_kg: f64,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = crate::schema::objects)]
pub struct UpdateObjectPayload {
    pub measured_at: NaiveDateTime,
    pub object_kg: f64,
}

pub fn create_object(
    conn: &mut PgConnection,
    create_object_payload: &CreateObjectPayload,
) -> Result<Object, diesel::result::Error> {
    let inserted_object = diesel::insert_into(objects::table)
        .values(create_object_payload)
        .returning(objects::all_columns)
        .get_result(conn)?;

    info!("create_object results: \n{:?}", inserted_object);
    Ok(inserted_object)
}

pub fn read_objects(conn: &mut PgConnection) -> Result<Vec<Object>, diesel::result::Error> {
    use crate::schema::objects::dsl::*;

    let results: Result<Vec<Object>, diesel::result::Error> = objects.limit(100).load(conn);

    info!("read_objects results: \n{:?}", results);
    results
}

pub fn update_object(
    conn: &mut PgConnection,
    target_object_id: ID,
    update_object_payload: &UpdateObjectPayload,
) -> Result<Object, diesel::result::Error> {
    use crate::schema::objects::dsl::*;

    let target = objects.filter(object_id.eq(target_object_id));
    update(target).set(update_object_payload).execute(conn)?;
    // TODO: How do I return the updated object without re-querying?
    let updated_object = objects
        .filter(object_id.eq(target_object_id))
        .get_result(conn)?;

    info!("update_object results: \n{:?}", updated_object);
    Ok(updated_object)
}

pub fn delete_object(
    conn: &mut PgConnection,
    target_object_id: ID,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::objects::dsl::*;

    let num_deleted =
        diesel::delete(objects.filter(object_id.eq(target_object_id))).execute(conn)?;

    info!("delete_object results: \n{:?}", num_deleted);

    if num_deleted != 1 {
        return Err(diesel::result::Error::RollbackTransaction);
    }

    Ok(num_deleted)
}

#[cfg(test)]
mod tests {

    use diesel::r2d2::{self, ConnectionManager, PooledConnection};

    use super::*;

    fn setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn get_test_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder().build(manager).unwrap();
        pool.get().unwrap()
    }

    fn create_object_for_test(conn: &mut PgConnection) -> Object {
        let new_object = CreateObjectPayload {
            measured_at: NaiveDateTime::from_timestamp_opt(1635535802, 0)
                .expect("Failed to create NaiveDateTime from timestamp"),
            object_kg: 70.5,
        };
        create_object(conn, &new_object).unwrap()
    }

    #[test]
    fn test_create_object() {
        setup();
        let mut conn = get_test_connection();

        let new_object = CreateObjectPayload {
            measured_at: NaiveDateTime::from_timestamp_opt(1635535802, 0)
                .expect("Failed to create NaiveDateTime from timestamp"),
            object_kg: 70.5,
        };

        let result = create_object(&mut conn, &new_object);
        assert!(result.is_ok());
        let object = result.unwrap();
        assert_eq!(new_object.measured_at, object.measured_at);
        assert_eq!(new_object.object_kg, object.object_kg);
    }

    #[test]
    fn test_read_objects() {
        setup();
        let mut conn = get_test_connection();
        create_object_for_test(&mut conn); // Ensure there is at least one object in the database
        let result = read_objects(&mut conn);
        assert!(result.is_ok());
        let objects = result.unwrap();
        assert!(!objects.is_empty());
    }

    #[test]
    fn test_update_object() {
        setup();
        let mut conn = get_test_connection();
        let target_object_id = create_object_for_test(&mut conn).object_id;
        info!("{:?}", create_object_for_test(&mut conn));
        let update_payload = UpdateObjectPayload {
            measured_at: NaiveDateTime::from_timestamp_opt(1635535802, 0)
                .expect("Failed to create NaiveDateTime from timestamp"),
            object_kg: 71.0,
        };
        let result = update_object(&mut conn, target_object_id, &update_payload);
        assert!(result.is_ok());
        let updated_object = result.unwrap();
        assert_eq!(update_payload.measured_at, updated_object.measured_at);
        assert_eq!(update_payload.object_kg, updated_object.object_kg);
    }

    #[test]
    fn test_delete_object() {
        setup();
        let mut conn = get_test_connection();
        let target_object_id = create_object_for_test(&mut conn).object_id;
        let result = delete_object(&mut conn, target_object_id);
        assert!(result.is_ok());
        assert_eq!(1, result.unwrap());
    }
}
