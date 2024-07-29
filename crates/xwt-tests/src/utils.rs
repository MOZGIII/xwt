use xwt_core::prelude::*;

pub async fn connect<Endpoint>(
    endpoint: &Endpoint,
    url: &str,
) -> Result<ConnectSessionFor<Endpoint>, xwt_error::Connect<Endpoint>>
where
    Endpoint: xwt_core::endpoint::Connect,
{
    let connecting = endpoint
        .connect(url)
        .await
        .map_err(xwt_error::Connect::Connect)?;

    let session = connecting
        .wait_connect()
        .await
        .map_err(xwt_error::Connect::Connecting)?;

    Ok(session)
}

pub async fn ok_accepting<Accepting>(
    accepting: Accepting,
) -> Result<AcceptingSessionFor<Accepting>, xwt_error::Accepting<Accepting>>
where
    Accepting: xwt_core::endpoint::accept::Accepting,
    AcceptingSessionFor<Accepting>: xwt_core::base::Session,
{
    let request = accepting
        .wait_accept()
        .await
        .map_err(xwt_error::Accepting::Accepting)?;

    let session = request
        .ok()
        .await
        .map_err(xwt_error::Accepting::RequestOk)?;

    Ok(session)
}

pub async fn open_bi<Session>(
    session: &Session,
) -> Result<BiStreamsFor<Session>, xwt_error::OpenBi<Session>>
where
    Session: xwt_core::session::stream::OpenBi,
{
    let opening = session.open_bi().await.map_err(xwt_error::OpenBi::Open)?;
    let streams = opening
        .wait_bi()
        .await
        .map_err(xwt_error::OpenBi::Opening)?;

    Ok(streams)
}

pub async fn open_uni<Session>(
    session: &Session,
) -> Result<SendUniStreamFor<Session>, xwt_error::OpenUni<Session>>
where
    Session: xwt_core::session::stream::OpenUni,
{
    let opening = session.open_uni().await.map_err(xwt_error::OpenUni::Open)?;
    let stream = opening
        .wait_uni()
        .await
        .map_err(xwt_error::OpenUni::Opening)?;

    Ok(stream)
}
