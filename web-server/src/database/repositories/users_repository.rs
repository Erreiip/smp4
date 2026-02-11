use chrono::{NaiveDateTime};
use serde::{Serialize};
use sqlx::{self, Error, FromRow};
use crate::database::db_manager::Database;

#[derive(Clone, Serialize, FromRow)]
pub struct User {
	pub id: i32,
	pub username: String,
	pub email: String,
	pub password: String,
	pub created_at: NaiveDateTime,
	pub updated_at: NaiveDateTime,
}

pub struct UsersRepository {
	db: Database,
}

impl UsersRepository {
	pub fn new(db: Database) -> Self {
		UsersRepository { db }
	}

	pub async fn get_user_by_id(&self, id: i32) -> Result<User, Error> {
		sqlx::query_as::<_, User>("
			SELECT *
			FROM users
			WHERE id = $1
		")
			.bind(id)
			.fetch_one(&self.db.pool)
			.await
	}

	pub async fn get_user_by_username(&self, username: String) -> Result<User, Error> {
		sqlx::query_as::<_, User>("
			SELECT *
			FROM users 
			WHERE username = $1
		")
			.bind(username)
			.fetch_one(&self.db.pool)
			.await
	}

	pub async fn create_user(&self, username: String, email: String, password: String) -> Result<User, Error> {
		sqlx::query_as::<_, User>("
			INSERT INTO users (username, email, password)
			VALUES ($1, $2, $3)
			RETURNING *
		")
			.bind(username)
			.bind(email)
			.bind(password)
			.fetch_one(&self.db.pool)
			.await
	}
}