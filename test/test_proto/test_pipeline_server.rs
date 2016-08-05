use support::mock;
use tokio;
use tokio::proto::pipeline;
use tokio::reactor::{self, Reactor};
use futures;
use std::io;

type Frame = pipeline::Frame<&'static str, io::Error>;

#[test]
fn test_immediate_echo() {
    ::env_logger::init().unwrap();

    let r = Reactor::default().unwrap();
    let h = r.handle();

    let (mock, new_transport) = mock::transport::<Frame, Frame>();

    // Spawn the reactor
    r.spawn();

    h.oneshot(move || {
        let transport = try!(new_transport.new_transport());

        let service = tokio::simple_service(|req: &'static str| {
            futures::finished::<&'static str, io::Error>(req)
        });

        let dispatch = try!(pipeline::Server::new(service, transport));

        try!(reactor::schedule(dispatch));

        Ok(())
    });

    mock.send(pipeline::Frame::Done);

    // Assert the mock is dropped
    mock.assert_drop();

    // mock.send(pipeline::Frame::Done);
    //
    // assert!(mock.did_flush())
    //
    // assert!(mock.did_not_flush(millis(30)));
    //
    // assert_eq!("foo", mock.next_write())
    //
    // assert_eq!("foo", mock.next_write_after_flushes(3));

    // mock.did_write(...)
    //
    // mock.accept_write(n)

    // assert!(mock.did_flush());
    // assert!(mock.did_write(...));
    // assert!(mock.did_not_write());
    // mock.is_writable();


    h.shutdown();
}
