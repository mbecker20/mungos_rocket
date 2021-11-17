// must have mungos and rocket with json feature enabled to use this macro

#[macro_export]
macro_rules! mungos_routes {
	($database:expr, $collection:expr, $type_name:ty) => {
		{
			use mungos::Mungos;
			use rocket::{State, serde::json::Json};

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
						.unwrap()
				)
			}

			#[patch("/<id>", data = "<data>")]
			async fn update(
				id: &str,
				mungos: &State<Mungos>,
				data: Json<$type_name>,
			) -> Json<$type_name> {
				Json(mungos.collection($database, $collection).update_one(id, data.into_inner())
					.await
					.unwrap())
			}

			#[post("/", data = "<data>")]
			async fn create(mungos: &State<Mungos>, data: Json<$type_name>) -> String {
				mungos.collection($database, $collection).create_one(data.into_inner())
					.await
					.unwrap()
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