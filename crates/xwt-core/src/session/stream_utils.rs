use super::stream;

pub type BiStreamOpeningErrorFor<T> = <<T as stream::OpenBi>::Opening as stream::OpeningBi>::Error;

pub type UniStreamOpeningErrorFor<T> =
    <<T as stream::OpenUni>::Opening as stream::OpeningUni>::Error;

pub type SendStreamFor<T> = <T as stream::SendSpec>::SendStream;

pub type RecvStreamFor<T> = <T as stream::RecvSpec>::RecvStream;

pub type SendUniStreamFor<T> =
    SendStreamFor<<<T as stream::OpenUni>::Opening as stream::OpeningUni>::Streams>;

pub type RecvUniStreamFor<T> =
    RecvStreamFor<<<T as stream::OpenUni>::Opening as stream::OpeningUni>::Streams>;

pub type BiStreamsFor<T> =
    stream::TupleFor<<<T as stream::OpenBi>::Opening as stream::OpeningBi>::Streams>;
