//! Generates default rocket REST routes for a collection.
//! 
//! Must be used along with mungos, rocket with the json feature enabled, and probably serde. 
//! Rocket must .manage an instance of Mungos.
//! 
//! The macro is accessed as ``` mungos_routes!("database name", "collection name", SerializableType) ```, 
//! where the database and collection names refer to your mongoDB database, and the SerializableType
//! is the type of the collection schema. 
//! 
//! # Examples
//! 
//! ```
//! #[macro_use]
//! extern crate rocket; // the rocket macros need to be available
//! #[macro_use]
//! extern crate mungos_rocket;
//! use mungos::Mungos;
//! 
//! #[derive(Debug, Serialize, Deserialize)]
//! struct SerializableType {
//! 	field: String,
//! }
//! 
//! rocket.build()
//! 	.manage(Mungos::new(...))
//! 	.mount("/", mungos_routes!("db name", "collection name", SerializableType) )
//! ```
//! 
//! The routes generated are: 
//! 	
//! 	- get " / " : get full collection
//! 
//! 	- get " /id " : get one document by its "_id" field
//! 
//! 	- post " / ", data : adds one document to the collection, specified by the struct in data
//! 
//! 	- patch " /id ", data : updates one document, specified by it's "_id" field, replacing it with the struct in data
//! 
//! 	- delete " /id " : deletes one document, specified by it's "_id" field
//!

#[macro_export]
macro_rules! mungos_routes {
	($database:expr, $collection:expr, $type_name:ty) => {
		{
			use mungos::{Mungos, Update};
			use rocket::{State, serde::json::Json, http::Status};

			#[get("/")]
			async fn get_all(mungos: &State<Mungos>) -> Json<Vec<$type_name>> {
				Json(
					mungos.collection($database, $collection).get_full_collection()
						.await
						.unwrap(),
				)
			}

			#[get("/<id>")]
			async fn get(id: &str, mungos: &State<Mungos>) -> Json<$type_name> {
				Json(
					mungos.collection($database, $collection).get_one(id)
						.await
						.unwrap()
				)
			}

			#[post("/", data = "<data>")]
			async fn create(mungos: &State<Mungos>, data: Json<$type_name>) -> String {
				mungos.collection($database, $collection).create_one(data.into_inner())
					.await
					.unwrap()
			}

			#[patch("/<id>", data = "<data>")]
			async fn update(
				id: &str,
				mungos: &State<Mungos>,
				data: Json<$type_name>,
			) -> Status {
				mungos.collection($database, $collection).update_one(id, Update::Regular(data.into_inner()))
					.await
					.unwrap();
				Status::Ok
			}

			#[delete("/<id>")]
			async fn delete(id: &str, mungos: &State<Mungos>) -> String {
				mungos.collection::<$type_name>($database, $collection).delete_one(id)
					.await
					.unwrap()
			}

			routes![get_all, get, update, create, delete]
		}
	};
}
