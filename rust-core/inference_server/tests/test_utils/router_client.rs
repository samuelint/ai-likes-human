use std::convert::Infallible;

use super::create_test_router;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use bytes::Bytes;
use http_body_util::BodyExt; // for `collect`
use serde::{Deserialize, Serialize};

use tower::ServiceExt; // for `oneshot`

#[allow(dead_code)]
pub struct RouterClient {
    router: Box<Router>,
    base_url: String,
}

impl RouterClient {
    pub fn new(router: axum::Router, base_url: &str) -> Self {
        Self {
            router: Box::new(router),
            base_url: base_url.to_string(),
        }
    }

    #[allow(dead_code)]
    pub async fn from_app(base_url: &str) -> Self {
        let router = create_test_router().await;

        Self::new(router, base_url)
    }

    #[allow(dead_code)]
    pub async fn post_as_bytes<TRequestBody>(
        &self,
        path: &str,
        body: &TRequestBody,
    ) -> Result<(Bytes, StatusCode), Infallible>
    where
        TRequestBody: ?Sized + Serialize,
    {
        let body_json = serde_json::to_string(&body).unwrap();
        let router = self.router.clone();

        let response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .header("Content-Type", "application/json")
                    .uri(format!("{}{}", self.base_url, path))
                    .body(Body::from(body_json))
                    .unwrap(),
            )
            .await?;

        let status: StatusCode = response.status();
        let body = response.into_body().collect().await.unwrap().to_bytes();

        Ok((body, status))
    }

    #[allow(dead_code)]
    pub async fn post_as_value<TRequestBody>(
        &self,
        path: &str,
        body: &TRequestBody,
    ) -> Result<(serde_json::Value, StatusCode), Infallible>
    where
        TRequestBody: ?Sized + Serialize,
    {
        self.post(path, body).await
    }

    #[allow(dead_code)]
    pub async fn post<TRequestBody, TResponseBody>(
        &self,
        path: &str,
        body: &TRequestBody,
    ) -> Result<(TResponseBody, StatusCode), Infallible>
    where
        TRequestBody: ?Sized + Serialize,
        for<'de> TResponseBody: Deserialize<'de>,
    {
        let (bytes, status) = self.post_as_bytes(path, body).await.unwrap();
        let body: TResponseBody = serde_json::from_slice(&bytes).unwrap();

        Ok((body, status))
    }

    #[allow(dead_code)]
    pub async fn get_as_bytes(&self, path: &str) -> Result<(Bytes, StatusCode), Infallible> {
        let router = self.router.clone();

        let response = router
            .oneshot(
                Request::builder()
                    .method("GET")
                    .header("Content-Type", "application/json")
                    .uri(format!("{}{}", self.base_url, path))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await?;

        let status: StatusCode = response.status();
        let bytes = response.into_body().collect().await.unwrap().to_bytes();

        Ok((bytes, status))
    }

    #[allow(dead_code)]
    pub async fn get<TResponseBody>(
        &self,
        path: &str,
    ) -> Result<(TResponseBody, StatusCode), Infallible>
    where
        for<'de> TResponseBody: Deserialize<'de>,
    {
        let (bytes, status) = self.get_as_bytes(path).await.unwrap();
        let body: TResponseBody = serde_json::from_slice(&bytes).unwrap();

        Ok((body, status))
    }

    #[allow(dead_code)]
    pub async fn get_as_value(
        &self,
        path: &str,
    ) -> Result<(serde_json::Value, StatusCode), Infallible> {
        self.get(path).await
    }
}
