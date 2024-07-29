use std::future::Future;

use typle::typle;

pub trait HandleSession: Send + 'static {
    type Error: std::error::Error + Send + Sync + 'static;

    fn handle_session(
        self,
        connection: std::sync::Arc<wtransport::Connection>,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;
}

impl<F, Fut, E> HandleSession for F
where
    F: Send + 'static,
    F: FnOnce(std::sync::Arc<wtransport::Connection>) -> Fut,
    Fut: Future<Output = Result<(), E>> + Send,
    E: std::error::Error + Send + Sync + 'static,
{
    type Error = E;

    async fn handle_session(
        self,
        connection: std::sync::Arc<wtransport::Connection>,
    ) -> Result<(), Self::Error> {
        (self)(connection).await
    }
}

pub trait SpawnHandleSession: Send + 'static {
    fn spawn_handle_session(
        self,
        joinset: &mut tokio::task::JoinSet<()>,
        session: std::sync::Arc<wtransport::Connection>,
    );
}

impl<T> SpawnHandleSession for T
where
    T: HandleSession,
{
    fn spawn_handle_session(
        self,
        joinset: &mut tokio::task::JoinSet<()>,
        session: std::sync::Arc<wtransport::Connection>,
    ) {
        joinset.spawn(async move {
            let what = std::any::type_name::<T>();
            tracing::info!(message = "serving", %what);
            let fut = self.handle_session(session);
            if let Err(error) = fut.await {
                tracing::error!(message = "error while serving", %error, %what);
            }
            tracing::info!(message = "done serving", %what);
        });
    }
}

#[typle(Tuple for 1..=5)]
impl<T> SpawnHandleSession for T
where
    T: Tuple,
    T<_>: SpawnHandleSession,
{
    fn spawn_handle_session(
        self,
        joinset: &mut tokio::task::JoinSet<()>,
        connection: std::sync::Arc<wtransport::Connection>,
    ) {
        for typle_index!(i) in 0..T::LEN {
            {
                let connection = std::sync::Arc::clone(&connection);
                self[[i]].spawn_handle_session(joinset, connection);
            }
        }
    }
}

pub async fn handle_session_with<T: SpawnHandleSession>(
    handler: T,
    session: std::sync::Arc<wtransport::Connection>,
) {
    let mut joinset = tokio::task::JoinSet::new();

    handler.spawn_handle_session(&mut joinset, std::sync::Arc::clone(&session));

    session.closed().await;

    tracing::info!(message = "connection is closing");

    while let Some(result) = joinset.join_next().await {
        if let Err(panic) = result {
            tracing::error!(message = "panic in the connection task", %panic);
        }
    }

    tracing::info!(message = "connection tasks are finished");
}

pub trait HandleSessionRequest {
    fn handle_session_request(
        self,
        session_request: wtransport::endpoint::SessionRequest,
    ) -> impl Future<Output = Result<(), wtransport::error::ConnectionError>> + Send;
}

pub trait StaticHandleSessionRequest {
    fn handle_session_request(
        session_request: wtransport::endpoint::SessionRequest,
    ) -> impl Future<Output = Result<(), wtransport::error::ConnectionError>> + Send;
}

impl<T: StaticHandleSessionRequest> HandleSessionRequest for T {
    fn handle_session_request(
        self,
        session_request: wtransport::endpoint::SessionRequest,
    ) -> impl Future<Output = Result<(), wtransport::error::ConnectionError>> + Send {
        <T as StaticHandleSessionRequest>::handle_session_request(session_request)
    }
}

pub struct AcceptSessionRequestWith<T: SpawnHandleSession>(pub T);

impl<T: SpawnHandleSession> HandleSessionRequest for AcceptSessionRequestWith<T> {
    async fn handle_session_request(
        self,
        session_request: wtransport::endpoint::SessionRequest,
    ) -> Result<(), wtransport::error::ConnectionError> {
        tracing::info!(message = "accepting incoming session");
        let session = session_request.accept().await?;

        tracing::info!(message = "new session accepted");

        let session = std::sync::Arc::new(session);

        handle_session_with(self.0, session).await;

        Ok(())
    }
}

pub trait RouteSession: Send {
    const PATH: &'static str;

    fn handler() -> impl HandleSessionRequest;
}

#[typle(Tuple for 1..=5)]
impl<T> StaticHandleSessionRequest for T
where
    T: Tuple,
    T<_>: RouteSession,
{
    async fn handle_session_request(
        session_request: wtransport::endpoint::SessionRequest,
    ) -> Result<(), wtransport::error::ConnectionError> {
        let path = session_request.path();

        for typle_index!(i) in 0..T::LEN {
            if T::<{ i }>::PATH == path {
                tracing::info!(message = "routing session on a known path", %path);
                let handler = T::<{ i }>::handler();
                return handler.handle_session_request(session_request).await;
            }
        }

        tracing::info!(message = "rejecting incoming session due to path mismatch");
        session_request.not_found().await;
        Ok(())
    }
}

pub async fn route<T: StaticHandleSessionRequest>(
    session_request: wtransport::endpoint::SessionRequest,
) -> Result<(), wtransport::error::ConnectionError> {
    <T as StaticHandleSessionRequest>::handle_session_request(session_request).await
}
