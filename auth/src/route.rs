pub fn route(r: http::Request<bytes::Bytes>) -> http::Response<bytes::Bytes> {
    use auth::handlers;
    use auth::layout::Auth;
    use auth::urls::Route;
    use ft_sdk::Layout;

    match Into::<Route>::into(r.uri().path()) {
        Route::CreateAccount => Auth::action::<handlers::CreateAccount>(r),
        Route::Login => todo!(),
        Route::Logout => todo!(),
        Route::EmailConfirmationSent => todo!(),
        Route::ConfirmEmail => todo!(),
        Route::ResendConfirmationEmail => todo!(),

        Route::Onboarding => todo!(),

        Route::ForgotPasswordSuccess => todo!(),
        Route::ForgotPassword => todo!(),
        Route::SetPassword => todo!(),
        Route::SetPasswordSuccess => todo!(),

        Route::GithubLogin => todo!(),
        Route::GithubCallback => todo!(),

        Route::Invalid => todo!(),
    }
}

// #[tracing::instrument(skip_all)]
// pub async fn handle_auth(
//     req: fastn_core::http::Request,
//     req_config: &mut fastn_core::RequestConfig,
//     config: &fastn_core::Config,
// ) -> fastn_core::Result<fastn_core::http::Response> {
//     use fastn_core::auth::Route;
//
//     let next = req.q("next", "/".to_string())?;
//
//     let pool = fastn_core::db::pool(&req_config.config.ds)
//         .await
//         .as_ref()
//         .map_err(|e| fastn_core::Error::DatabaseError {
//             message: format!("Failed to get connection to db. {:?}", e),
//         })?;
//
//     match Into::<Route>::into(req.path()) {
//         Route::Login => fastn_core::auth::email_password::login(req_config, pool, next).await,
//         Route::GithubLogin => {
//             fastn_core::auth::github::login(&req_config.config.ds, &req, next).await
//         }
//         Route::GithubCallback => {
//             fastn_core::auth::github::callback(&req, &req_config.config.ds, pool, next).await
//         }
//         Route::Logout => fastn_core::auth::logout(&req, &req_config.config.ds, pool, next).await,
//         Route::CreateAccount => {
//             fastn_core::auth::email_password::create_account(req_config, pool, next).await
//         }
//         Route::EmailConfirmationSent => {
//             fastn_core::auth::email_password::email_confirmation_sent(req_config).await
//         }
//         Route::ConfirmEmail => {
//             fastn_core::auth::email_password::confirm_email(req_config, pool, next).await
//         }
//         Route::ResendConfirmationEmail => {
//             fastn_core::auth::email_password::resend_confirmation_email(
//                 &req, req_config, pool, next,
//             )
//             .await
//         }
//         Route::Onboarding => {
//             fastn_core::auth::email_password::onboarding(&req, req_config, config, next).await
//         }
//         Route::ForgotPassword => {
//             fastn_core::auth::email_password::forgot_password_request(req_config, pool, next).await
//         }
//         Route::ForgotPasswordSuccess => {
//             fastn_core::auth::email_password::forgot_password_request_success(req_config).await
//         }
//         Route::SetPassword => {
//             fastn_core::auth::email_password::set_password(req_config, pool, next).await
//         }
//         Route::SetPasswordSuccess => {
//             fastn_core::auth::email_password::set_password_success(req_config).await
//         }
//         Route::Invalid => Ok(fastn_core::not_found!("route not found: {}", req.path())),
//     }
// }
