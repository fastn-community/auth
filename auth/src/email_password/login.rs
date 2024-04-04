pub(crate) async fn login(
    req_config: &mut fastn_core::RequestConfig,
    db_pool: &fastn_core::db::PgPool,
    next: String,
) -> fastn_core::Result<fastn_core::http::Response> {
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    if req_config.request.method() != "POST" {
        // TODO: if user is logged in redirect to next

        let main = fastn_core::Document {
            package_name: req_config.config.package.name.clone(),
            id: fastn_core::auth::Route::Login.to_string(),
            content: fastn_core::auth::email_password::login_ftd().to_string(),
            parent_path: fastn_ds::Path::new("/"),
        };

        let resp = fastn_core::package::package_doc::read_ftd(req_config, &main, "/", false, false)
            .await?;

        return Ok(resp.into());
    }

    #[derive(serde::Deserialize, validator::Validate, Debug)]
    struct Payload {
        username: String,
        password: String,
    }

    let payload = req_config.request.json::<Payload>();

    if let Err(e) = payload {
        return fastn_core::http::user_err(
            vec![("payload".into(), vec![format!("invalid payload: {:?}", e)])],
            fastn_core::http::StatusCode::OK,
        );
    }

    let payload = payload.unwrap();

    let mut errors = Vec::new();

    if payload.username.is_empty() {
        errors.push(("username".into(), vec!["username/email is required".into()]));
    }

    if payload.password.is_empty() {
        errors.push(("password".into(), vec!["password is required".into()]));
    }

    if !errors.is_empty() {
        return fastn_core::http::user_err(errors, fastn_core::http::StatusCode::OK);
    }

    let mut conn = db_pool
        .get()
        .await
        .map_err(|e| fastn_core::Error::DatabaseError {
            message: format!("Failed to get connection to db. {:?}", e),
        })?;

    let query = fastn_core::schema::fastn_user::table
        .filter(fastn_core::schema::fastn_user::username.eq(&payload.username))
        .or_filter(
            fastn_core::schema::fastn_user::email.eq(fastn_core::utils::citext(&payload.username)),
        )
        .select(fastn_core::auth::FastnUser::as_select());

    let user: Option<fastn_core::auth::FastnUser> = query.first(&mut conn).await.optional()?;

    if user.is_none() {
        return fastn_core::http::user_err(
            vec![("username".into(), vec!["invalid email/username".into()])],
            fastn_core::http::StatusCode::OK,
        );
    }

    let user = user.expect("expected user to be Some");

    // OAuth users don't have password
    if user.password.is_empty() {
        // TODO: create feature to ask if the user wants to convert their account to an email
        // password
        // or should we redirect them to the oauth provider they used last time?
        // redirecting them will require saving the method they used to login which de don't atm
        return fastn_core::http::user_err(
            vec![("username".into(), vec!["invalid username".into()])],
            fastn_core::http::StatusCode::OK,
        );
    }

    let parsed_hash = argon2::PasswordHash::new(&user.password)
        .map_err(|e| fastn_core::Error::generic(format!("failed to parse hashed password: {e}")))?;

    let password_match = argon2::PasswordVerifier::verify_password(
        &argon2::Argon2::default(),
        payload.password.as_bytes(),
        &parsed_hash,
    );

    if password_match.is_err() {
        return fastn_core::http::user_err(
            vec![(
                "password".into(),
                vec!["incorrect username/password".into()],
            )],
            fastn_core::http::StatusCode::OK,
        );
    }

    let now = chrono::Utc::now();

    // TODO: session should store device that was used to login (chrome desktop on windows)
    let session_id: i64 = diesel::insert_into(fastn_core::schema::fastn_auth_session::table)
        .values((
            fastn_core::schema::fastn_auth_session::user_id.eq(&user.id),
            fastn_core::schema::fastn_auth_session::created_at.eq(now),
            fastn_core::schema::fastn_auth_session::updated_at.eq(now),
        ))
        .returning(fastn_core::schema::fastn_auth_session::id)
        .get_result(&mut conn)
        .await?;

    tracing::info!("session created. session id: {}", &session_id);

    // client has to 'follow' this request
    // https://stackoverflow.com/a/39739894
    fastn_core::auth::set_session_cookie_and_redirect_to_next(
        &req_config.request,
        &req_config.config.ds,
        session_id,
        next,
    )
    .await
}
