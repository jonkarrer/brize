use crate::SetupError;
use crate::schema::{team_members, teams, users};
use bcrypt::{DEFAULT_COST, hash, verify};
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct User {
    id: i32,
    name: Option<String>,
    email: String,
    role: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
struct NewUser {
    name: String,
    email: String,
    password_hash: String,
    role: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = teams)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Team {
    id: i32,
    name: String,
}

#[derive(Insertable)]
#[diesel(table_name = teams)]
struct NewTeam {
    name: String,
}

#[derive(Insertable)]
#[diesel(table_name = team_members)]
struct NewTeamMember {
    team_id: i32,
    user_id: i32,
    role: String,
}

pub fn run(db_url: &str) -> Result<(), SetupError> {
    let mut conn =
        PgConnection::establish(db_url).map_err(|_| SetupError("DB Conn failed".to_string()))?;

    let new_user = NewUser {
        name: "admin".to_string(),
        email: "admin@test.com".to_string(),
        password_hash: hash_password("password").unwrap(),
        role: "admin".to_string(),
    };

    let new_team = NewTeam {
        name: "Test Team".to_string(),
    };

    let user = diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .expect("Failed to insert user");

    let team = diesel::insert_into(teams::table)
        .values(new_team)
        .returning(Team::as_returning())
        .get_result(&mut conn)
        .expect("Failed to insert Team");

    let new_member = NewTeamMember {
        user_id: user.id,
        team_id: team.id,
        role: "admin".to_string(),
    };

    diesel::insert_into(team_members::table)
        .values(&new_member)
        .execute(&mut conn)
        .expect("Failed to inster team member");

    Ok(())
}

fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

fn verify_password(password: &str, hashed: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hashed)
}
