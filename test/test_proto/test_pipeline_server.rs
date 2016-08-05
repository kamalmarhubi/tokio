use support::{self, mock};
use tokio::{self, Service};
use tokio::proto::pipeline::{self, Frame};
use tokio::reactor::{self, Reactor};
use futures::{self, Future};
use std::io;

#[test]
fn test_immediate_done() {
    let service = tokio::simple_service(|req| {
        futures::finished::<&'static str, io::Error>(req)
    });

    run(service, |mock| {
        mock.send(pipeline::Frame::Done);

        // Assert the mock is dropped
        mock.allow_and_assert_drop();
    });
}

#[test]
fn test_immediate_writable_echo() {
    let service = tokio::simple_service(|req| {
        assert_eq!("hello", req);
        futures::finished::<&'static str, io::Error>(req)
    });

    run(service, |mock| {
        mock.allow_write();
        mock.send(pipeline::Frame::Message("hello"));
        assert_eq!("hello", mock.next_write().unwrap_msg());
    });
}

/// Setup a reactor running a pipeline::Server with the given service and a
/// mock transport. Yields the mock transport handle to the function.
fn run<S, F>(service: S, f: F)
    where S: Service<Req = &'static str, Resp = &'static str, Error = io::Error>,
          F: FnOnce(mock::TransportHandle<Frame<&'static str, io::Error>, Frame<&'static str, io::Error>>),
{
    let _ = ::env_logger::init();
    let r = Reactor::default().unwrap();
    let h = r.handle();

    let (mock, new_transport) = mock::transport();

    // Spawn the reactor
    r.spawn();

    h.oneshot(move || {
        let transport = try!(new_transport.new_transport());
        let dispatch = try!(pipeline::Server::new(service, transport));

        try!(reactor::schedule(dispatch));

        Ok(())
    });

    f(mock);


    h.shutdown();
}
