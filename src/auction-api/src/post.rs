use crate::models::{NewAuction, NewUser, NewBid};
use crate::response::SuccessResponse;
use actix_web::{error::BlockingError, post, web, HttpResponse};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{r2d2, PgConnection};

use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::DatabaseError;

use crate::actions;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

type PoolConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn get_conn(pool: web::Data<DbPool>) -> Result<PoolConnection, HttpResponse> {
    pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })
}

#[post("/api/register")]
pub async fn register_user(
    pool: web::Data<DbPool>,
    form: web::Json<NewUser>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = get_conn(pool)?;

    let new_user = form.into_inner();

    let res = web::block(move || actions::insert_new_user(&conn, &new_user)).await;
    match res {
        Ok(_) => Ok(HttpResponse::Ok().json(SuccessResponse { success: true })),
        Err(err) => match err {
            BlockingError::Error(diesel_error) => match diesel_error {
                DatabaseError(UniqueViolation, _msg) => {
                    let res = HttpResponse::Conflict().json(SuccessResponse { success: false });
                    Ok(res)
                }
                _ => Ok(HttpResponse::InternalServerError().finish()),
            },
            BlockingError::Canceled => Ok(HttpResponse::InternalServerError().finish()),
        },
    }
}

#[post("/api/login")]
pub async fn login_user(
    pool: web::Data<DbPool>,
    form: web::Json<NewUser>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = get_conn(pool)?;

    let user = form.into_inner();
    let user_moved = user.clone();
    let res = web::block(move || actions::find_user_by_name(&conn, user_moved.name())).await;
    let login = match res {
        Ok(db_user) => db_user.map_or(false, |u| u.password() == user.password()),
        _ => {
            HttpResponse::InternalServerError().finish();
            false
        }
    };
    Ok(HttpResponse::Ok().json(SuccessResponse { success: login }))
}

#[post("/api/auctions")]
pub async fn create_auction(
    pool: web::Data<DbPool>,
    form: web::Json<NewAuction>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = get_conn(pool)?;

    let auction = form.into_inner();

    let res = web::block(move || actions::insert_new_auction(&conn, &auction)).await;

    match res {
        Ok(_) => Ok(HttpResponse::Ok().json(SuccessResponse { success: true })),
        Err(err) => match err {
            BlockingError::Error(diesel_error) => match diesel_error {
                DatabaseError(UniqueViolation, _msg) => {
                    let res = HttpResponse::Conflict().json(SuccessResponse { success: false });
                    Ok(res)
                }
                _ => Ok(HttpResponse::InternalServerError().finish()),
            },
            BlockingError::Canceled => Ok(HttpResponse::InternalServerError().finish()),
        },
    }
}

#[post("/api/bids")]
pub async fn create_bid(
    pool: web::Data<DbPool>,
    form: web::Json<NewBid>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = get_conn(pool)?;

    let bid = form.into_inner();

    let res = web::block(move || actions::insert_new_bid(&conn, &bid)).await;

    match res {
        Ok(_) => Ok(HttpResponse::Ok().json(SuccessResponse { success: true })),
        Err(err) => match err {
            BlockingError::Error(diesel_error) => match diesel_error {
                DatabaseError(UniqueViolation, _msg) => {
                    let res = HttpResponse::Conflict().json(SuccessResponse { success: false });
                    Ok(res)
                }
                _ => Ok(HttpResponse::InternalServerError().finish()),
            },
            BlockingError::Canceled => Ok(HttpResponse::InternalServerError().finish()),
        },
    }
}