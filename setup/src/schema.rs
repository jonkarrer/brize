// @generated automatically by Diesel CLI.

diesel::table! {
    activity_logs (id) {
        id -> Int4,
        team_id -> Int4,
        user_id -> Nullable<Int4>,
        action -> Text,
        timestamp -> Timestamp,
        #[max_length = 45]
        ip_address -> Nullable<Varchar>,
    }
}

diesel::table! {
    invitations (id) {
        id -> Int4,
        team_id -> Int4,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 50]
        role -> Varchar,
        invited_by -> Int4,
        invited_at -> Timestamp,
        #[max_length = 20]
        status -> Varchar,
    }
}

diesel::table! {
    team_members (id) {
        id -> Int4,
        user_id -> Int4,
        team_id -> Int4,
        #[max_length = 50]
        role -> Varchar,
        joined_at -> Timestamp,
    }
}

diesel::table! {
    teams (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        stripe_customer_id -> Nullable<Text>,
        stripe_subscription_id -> Nullable<Text>,
        stripe_product_id -> Nullable<Text>,
        #[max_length = 50]
        plan_name -> Nullable<Varchar>,
        #[max_length = 20]
        subscription_status -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Varchar,
        password_hash -> Text,
        #[max_length = 20]
        role -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(activity_logs -> teams (team_id));
diesel::joinable!(activity_logs -> users (user_id));
diesel::joinable!(invitations -> teams (team_id));
diesel::joinable!(invitations -> users (invited_by));
diesel::joinable!(team_members -> teams (team_id));
diesel::joinable!(team_members -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    activity_logs,
    invitations,
    team_members,
    teams,
    users,
);
