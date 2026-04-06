use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use super::PgConnection;
use crate::{
    schema::users,
    models::users::*,
    errors::db_errors::*,
    constants::LIMIT_OF_QUERY_RESULTS
};

pub enum SortUser {
    ByNicknameASC,
    ByNicknameDESC,
    ByCreationTimeASC,
    ByCreationTimeDESC
}

pub async fn get_db_user(connection: &mut PgConnection, nickname: &str) -> Result<DbUser, GetError> {
    users::table
        .filter(users::nickname.eq(nickname))
        .first::<DbUser>(connection)
        .await
        .map_err(|_| GetError::NotFound)
}

pub async fn get_user_by_id(connection: &mut PgConnection, user_id: i32) -> Result<DbGetUser, GetError> {
    users::table
        .find(user_id)
        .select(DbGetUser::as_select())
        .first::<DbGetUser>(connection)
        .await
        .map_err(|_| GetError::NotFound)
}

pub async fn get_user_by_nickname(connection: &mut PgConnection, nickname: &str) -> Result<DbGetUser, GetError> {
    users::table
        .filter(users::nickname.eq(nickname))
        .select(DbGetUser::as_select())
        .first::<DbGetUser>(connection)
        .await
        .map_err(|_| GetError::NotFound)
}

pub async fn get_users_with_sort(connection: &mut PgConnection, offset: i64, sorting: SortUser) -> Result<Vec<DbGetUser>, GetError> {
    match sorting {
        SortUser::ByNicknameASC => {
            users::table
                .order_by(users::nickname.asc())
                .limit(LIMIT_OF_QUERY_RESULTS)
                .offset(offset)
                .select(DbGetUser::as_select())
                .load::<DbGetUser>(connection)
                .await
                .map_err(|_| GetError::Internal)
        }
        SortUser::ByNicknameDESC => {
            users::table
                .order_by(users::nickname.desc())
                .limit(LIMIT_OF_QUERY_RESULTS)
                .offset(offset)
                .select(DbGetUser::as_select())
                .load::<DbGetUser>(connection)
                .await
                .map_err(|_| GetError::Internal)
        }
        SortUser::ByCreationTimeASC => {
            users::table
                .order_by(users::id.asc())
                .limit(LIMIT_OF_QUERY_RESULTS)
                .offset(offset)
                .select(DbGetUser::as_select())
                .load::<DbGetUser>(connection)
                .await
                .map_err(|_| GetError::Internal)
        }
        SortUser::ByCreationTimeDESC => {
            users::table
                .order_by(users::id.desc())
                .limit(LIMIT_OF_QUERY_RESULTS)
                .offset(offset)
                .select(DbGetUser::as_select())
                .load::<DbGetUser>(connection)
                .await
                .map_err(|_| GetError::Internal)
        }
    }
}

pub async fn post_user(connection: &mut PgConnection, user: &DbPostUser) -> Result<DbGetUser, PostError> {
    diesel::insert_into(users::table)
        .values(user)
        .on_conflict(users::nickname)
        .do_nothing()
        .returning(DbGetUser::as_returning())
        .get_result(connection)
        .await
        .map_err(|err| {
            match err {
                // Вставленная запись не найдена -> произошёл конфликт с другой
                diesel::result::Error::NotFound => PostError::Conflict,
                _ => PostError::Internal,
            }
        })
}

pub async fn patch_user(connection: &mut PgConnection, user: &DbPatchUser) -> Result<DbGetUser, PatchError> {
    diesel::update(users::table.find(user.id))
        .set(user)
        .returning(DbGetUser::as_returning())
        .get_result(connection)
        .await
        .map_err(|err| {
            if let diesel::result::Error::DatabaseError (
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) = err
            {
                PatchError::Conflict
            } else if let diesel::result::Error::NotFound = err {
                PatchError::NotFound
            } else {
                PatchError::Internal
            }
        })
}
