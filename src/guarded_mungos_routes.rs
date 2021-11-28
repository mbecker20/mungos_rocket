//! Generates default rocket REST routes for a collection.
//! 
//! Must be used along with mungos, rocket with the json feature enabled, and probably serde. 
//! Rocket must .manage an instance of Mungos.
//! 
//! The macro is accessed as ``` mungos_routes!("database name", "collection name", SerializableType, RequestGuard) ```, 
//! where the database and collection names refer to your mongoDB database, and the SerializableType
//! is the type of the collection schema. RequestGuard refers to a rocket request guard struct that you want 
//! to guard your routes (such as authentication). 
//! 
//! # Examples
//! 
//! ```
//! #[macro_use]
//! extern crate rocket; // the rocket macros need to be available
//! #[macro_use]
//! extern crate mungos_rocket;
//! use mungos::Mungos;
//! use crate::AuthGuard;
//! 
//! #[derive(Debug, Serialize, Deserialize)]
//! struct SerializableType {
//! 	field: String,
//! }
//! 
//! rocket.build()
//! 	.manage(Mungos::new(...))
//! 	.mount("/", mungos_routes!("db name", "collection name", SerializableType, AuthGuard) )
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
macro_rules! guarded_mungos_routes {
	($database:expr, $collection:expr, $TypeName:ty, $RequestGuard:ty) => {
		{
			use mungos::Mungos;
			use rocket::{State, serde::json::Json};

			#[get("/")]
			async fn get_all(mungos: &State<Mungos>, _guard: RequestGuard) -> Json<Vec<$TypeName>> {
				Json(
					mungos.collection($database, $collection).get_full_collection()
						.await
						.unwrap(),
				)
			}

			#[get("/<id>")]
			async fn get(id: &str, mungos: &State<Mungos>, _guard: RequestGuard) -> Json<$TypeName> {
				Json(
					mungos.collection($database, $collection).get_one(id)
						.await
						.unwrap()
						.unwrap()
				)
			}

			#[post("/", data = "<data>")]
			async fn create(mungos: &State<Mungos>, data: Json<$TypeName>, _guard: RequestGuard) -> String {
				mungos.collection($database, $collection).create_one(data.into_inner())
					.await
					.unwrap()
			}

			#[patch("/<id>", data = "<data>")]
			async fn update(
				id: &str,
				mungos: &State<Mungos>,
				data: Json<$TypeName>,
        _guard: RequestGuard
			) -> Json<$TypeName> {
				Json(mungos.collection($database, $collection).update_one(id, data.into_inner())
					.await
					.unwrap())
			}

			#[delete("/<id>")]
			async fn delete(id: &str, mungos: &State<Mungos>, _guard: RequestGuard) -> String {
				mungos.collection::<$TypeName>($database, $collection).delete_one(id)
					.await
					.unwrap()
			}

			routes![get_all, get, update, create, delete]
		}
	};
}
