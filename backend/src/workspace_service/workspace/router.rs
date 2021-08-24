use crate::{
    routers::utils::parse_from_payload,
    workspace_service::workspace::{
        create_workspace,
        delete_workspace,
        read_workspace,
        update_workspace,
    },
};
use actix_identity::Identity;
use actix_web::{
    web::{Data, Payload},
    Error,
    HttpRequest,
    HttpResponse,
};
use flowy_net::errors::ServerError;
use flowy_workspace::protobuf::{
    CreateWorkspaceParams,
    DeleteWorkspaceParams,
    QueryWorkspaceParams,
    UpdateWorkspaceParams,
};
use sqlx::PgPool;

pub async fn create_handler(
    payload: Payload,
    pool: Data<PgPool>,
) -> Result<HttpResponse, ServerError> {
    let params: CreateWorkspaceParams = parse_from_payload(payload).await?;
    let resp = create_workspace(pool.get_ref(), params).await?;
    Ok(resp.into())
}

pub async fn read_handler(
    payload: Payload,
    pool: Data<PgPool>,
) -> Result<HttpResponse, ServerError> {
    let params: QueryWorkspaceParams = parse_from_payload(payload).await?;
    let resp = read_workspace(pool.get_ref(), params).await?;
    Ok(resp.into())
}

pub async fn delete_handler(
    payload: Payload,
    pool: Data<PgPool>,
) -> Result<HttpResponse, ServerError> {
    let params: DeleteWorkspaceParams = parse_from_payload(payload).await?;
    let resp = delete_workspace(pool.get_ref(), params).await?;
    Ok(resp.into())
}

pub async fn update_handler(
    payload: Payload,
    id: Identity,
    pool: Data<PgPool>,
) -> Result<HttpResponse, ServerError> {
    let params: UpdateWorkspaceParams = parse_from_payload(payload).await?;
    let resp = update_workspace(pool.get_ref(), params).await?;
    Ok(resp.into())
}