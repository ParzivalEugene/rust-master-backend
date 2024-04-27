use crate::{
    data::{
        enums::{Country, SessionStatus},
        models::{Config, Device, Server, Session},
        schema,
    },
    dto::session::SessionBy,
    enums::errors::internal::{to_internal, InternalError, SessionError},
};
use diesel::prelude::*;
use diesel::QueryDsl;
use uuid::Uuid;

pub struct ActiveSessionAndDeviceAndCountry {
    pub device_id: Uuid,
    pub country: Country,
}

impl SessionBy for ActiveSessionAndDeviceAndCountry {
    async fn get_session<'a>(
        &self,
        pool: &'a deadpool_diesel::postgres::Pool,
    ) -> Result<(Session, Device, Config, Server), InternalError> {
        let conn = pool.get().await.map_err(to_internal)?;
        let (device_id, country) = (self.device_id.clone(), self.country.clone());
        let result: Vec<(Session, Device, Config, Server)> = conn
            .interact(move |conn| {
                schema::session::table
                    .inner_join(schema::device::table)
                    .inner_join(schema::config::table.inner_join(schema::server::table))
                    .filter(schema::session::device_id.eq(device_id))
                    .filter(schema::session::status.eq(SessionStatus::Active))
                    .filter(schema::server::country.eq(country))
                    .select((
                        Session::as_select(),
                        Device::as_select(),
                        Config::as_select(),
                        Server::as_select(),
                    ))
                    .load::<(Session, Device, Config, Server)>(conn)
            })
            .await
            .map_err(to_internal)?
            .map_err(|_| InternalError::Internal)?;
        if result.len() != 1 {
            return Err(InternalError::SessionError(SessionError::SessionNotFound));
        }
        Ok(result.first().unwrap().clone())
    }
}