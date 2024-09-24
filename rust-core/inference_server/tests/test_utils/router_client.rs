use super::create_test_router;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use bytes::Bytes;
use futures::{Stream, StreamExt};
use http_body_util::BodyExt; // for `collect`
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{convert::Infallible, pin::Pin};
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
    ) -> Result<(Option<serde_json::Value>, StatusCode), Infallible>
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
    ) -> Result<(Option<TResponseBody>, StatusCode), Infallible>
    where
        TRequestBody: ?Sized + Serialize,
        for<'de> TResponseBody: Deserialize<'de>,
    {
        let (bytes, status) = self.post_as_bytes(path, body).await.unwrap();
        if bytes.is_empty() {
            return Ok((None, status));
        }

        let body: TResponseBody = serde_json::from_slice(&bytes).unwrap();

        Ok((Some(body), status))
    }

    #[allow(dead_code)]
    pub fn post_stream<TRequestBody>(
        &self,
        path: &str,
        body: &TRequestBody,
    ) -> Pin<Box<dyn Stream<Item = Result<String, axum::Error>> + Send>>
    where
        TRequestBody: ?Sized + Serialize,
    {
        let body_json = serde_json::to_string(&body).unwrap();
        let router = self.router.clone();
        let base_url = self.base_url.clone();
        let path = path.to_string();

        let stream = async_stream::stream! {
            let response = match router.oneshot(Request::builder()
                .method("POST")
                .header("Content-Type", "application/json")
                .header("Accept", "text/event-stream")
                .uri(format!("{}{}", base_url, path))
                .body(Body::from(body_json))
                .unwrap(),).await {
              Ok(response) => response,
              Err(_) => {
                  return;
              }
            };

            let mut stream = response.into_data_stream();
            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(chunk) => {
                        let text = String::from_utf8(chunk.to_vec()).unwrap();
                        yield Ok(text);
                    }
                    Err(e) => {
                        yield Err(e);

                        return;
                    },
                }
            };

        };

        Box::pin(stream)
    }

    #[allow(dead_code)]
    pub fn post_json_stream<TRequestBody, TResponseBody>(
        &self,
        path: &str,
        body: &TRequestBody,
    ) -> impl Stream<Item = Result<TResponseBody, axum::Error>>
    where
        TRequestBody: ?Sized + Serialize,
        TResponseBody: DeserializeOwned,
    {
        let stream = self.post_stream(path, body);

        let stream = stream.map(|item| match item {
            Ok(text) => {
                let text = text.trim_start_matches("data: ");
                let chunk: TResponseBody = serde_json::from_str(text).unwrap();
                Ok(chunk)
            }
            Err(e) => Err(axum::Error::new(e)),
        });

        stream
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
    ) -> Result<(Option<TResponseBody>, StatusCode), Infallible>
    where
        for<'de> TResponseBody: Deserialize<'de>,
    {
        let (bytes, status) = self.get_as_bytes(path).await.unwrap();

        if bytes.is_empty() {
            return Ok((None, status));
        }

        let body: TResponseBody = serde_json::from_slice(&bytes).unwrap();

        Ok((Some(body), status))
    }

    #[allow(dead_code)]
    pub async fn get_as_value(
        &self,
        path: &str,
    ) -> Result<(Option<serde_json::Value>, StatusCode), Infallible> {
        self.get(path).await
    }

    #[allow(dead_code)]
    pub async fn delete(&self, path: &str) -> Result<StatusCode, Infallible> {
        let router = self.router.clone();

        let response = router
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .header("Content-Type", "application/json")
                    .uri(format!("{}{}", self.base_url, path))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await?;

        let status: StatusCode = response.status();

        Ok(status)
    }
}
