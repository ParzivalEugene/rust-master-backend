// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ConfigStatus"))]
    pub struct ConfigStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "Country"))]
    pub struct Country;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "DeviceStatus"))]
    pub struct DeviceStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "OS"))]
    pub struct Os;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "SessionStatus"))]
    pub struct SessionStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "UserStatus"))]
    pub struct UserStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ConfigStatus;

    Config (id) {
        id -> Uuid,
        private_key -> Text,
        user_ip -> Text,
        server_id -> Uuid,
        status -> ConfigStatus,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Os;
    use super::sql_types::DeviceStatus;

    Device (id) {
        id -> Uuid,
        name -> Text,
        os -> Os,
        user_id -> Uuid,
        banned_at -> Nullable<Timestamp>,
        banned_till -> Nullable<Timestamp>,
        revoked_at -> Nullable<Timestamp>,
        status -> DeviceStatus,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Country;

    Server (id) {
        id -> Uuid,
        public_key -> Text,
        backend_uri -> Text,
        wireguard_uri -> Text,
        country -> Country,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SessionStatus;

    Session (id) {
        id -> Uuid,
        status -> SessionStatus,
        opened_at -> Timestamp,
        closed_at -> Nullable<Timestamp>,
        device_id -> Uuid,
        config_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserStatus;

    User (id) {
        id -> Uuid,
        email -> Text,
        password -> Text,
        banned_at -> Nullable<Timestamp>,
        banned_till -> Nullable<Timestamp>,
        status -> UserStatus,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(Config -> Server (server_id));
diesel::joinable!(Device -> User (user_id));
diesel::joinable!(Session -> Config (config_id));
diesel::joinable!(Session -> Device (device_id));

diesel::allow_tables_to_appear_in_same_query!(
    Config,
    Device,
    Server,
    Session,
    User,
);
