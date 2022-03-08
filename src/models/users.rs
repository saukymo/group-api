use sqlx::{query_as, PgPool};

use crate::models::models::{NewUser, User, Appointment};

impl User {

    pub async fn add_new_user(new_user: NewUser, pg_conn: &PgPool) -> tide::Result<User> {
        let created_user = query_as!(User, r#"INSERT INTO users (name, wx_open_id) VALUES ($1, $2) RETURNING user_id, name, wx_open_id
    "#, new_user.name, new_user.wx_open_id).fetch_one(pg_conn).await?;

        Ok(created_user)
    }

    // pub async fn find_user_by_open_id(wx_open_id: String, pg_conn: &PgPool) -> tide::Result<Option<User>> {
    //     let user = query_as!(User, r#"Select user_id, name, wx_open_id from users where wx_open_id=$1"#, wx_open_id).fetch_optional(pg_conn).await?;

    //     Ok(user)
    // }
    
    pub async fn get_users(pg_conn: &PgPool) -> tide::Result<Vec<User>> {
        let users = query_as!(User, r#"SELECT user_id, name, wx_open_id FROM users"#).fetch_all(pg_conn).await?;

        Ok(users)
    }

    pub async fn get_user_by_user_id(user_id: i32, pg_conn: &PgPool) -> tide::Result<Option<User>> {
        let user = query_as!(User, r#"SELECT user_id, name, wx_open_id FROM users where user_id=$1"#, user_id).fetch_optional(pg_conn).await?;

        Ok(user)
    }

    pub async fn get_users_by_proposal_id(proposal_id: i32, pg_conn: &PgPool) -> tide::Result<Vec<User>> {
        let user_ids = Appointment::get_appointments_by_proposal_id(proposal_id, pg_conn).await?
        .iter()
        .map(|x| x.user_id)
        .collect::<Vec<_>>();

        let users = query_as!(User, r#"
            SELECT 
                user_id, 
                name, 
                wx_open_id 
            FROM 
                users
            WHERE 
                user_id = ANY($1)"#, &user_ids[..]).fetch_all(pg_conn).await?;

        Ok(users)
    }
}