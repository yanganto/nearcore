use crate::concurrency::arc_mutex::ArcMutex;
use crate::concurrency::demux;

#[tokio::test]
async fn test_demux() {
    let demux = demux::Demux::new(demux::RateLimit { qps: 50., burst: 1 });
    for _ in 0..5 {
        let mut handles = vec![];
        for i in 0..1000 {
            let demux = demux.clone();
            handles.push(tokio::spawn(async move {
                let j = demux
                    .call(i, |is: Vec<u64>| async { is.into_iter().map(|i| i + 1).collect() })
                    .await
                    .unwrap();
                assert_eq!(i + 1, j);
            }));
        }
        for h in handles {
            h.await.unwrap();
        }
    }
}

#[test]
fn demux_runtime_dropped_before_call() {
    let r1 = tokio::runtime::Runtime::new().unwrap();
    let r2 = tokio::runtime::Runtime::new().unwrap();
    let demux = r1.block_on(async { demux::Demux::new(demux::RateLimit { qps: 1., burst: 1000 }) });
    drop(r1);
    let call = demux.call(0, |is: Vec<u64>| async { is });
    assert_eq!(Err(demux::ServiceStoppedError), r2.block_on(call));
}

#[test]
fn demux_runtime_dropped_during_call() {
    let r1 = tokio::runtime::Runtime::new().unwrap();
    let r2 = tokio::runtime::Runtime::new().unwrap();
    let demux = r1.block_on(async { demux::Demux::new(demux::RateLimit { qps: 1., burst: 1000 }) });

    // Start the call and pause.
    let (send, recv) = tokio::sync::oneshot::channel();
    let call = r2.spawn(demux.call(0, move |_: Vec<u64>| async {
        send.send(()).unwrap();
        std::future::pending::<Vec<()>>().await
    }));
    r2.block_on(recv).unwrap();

    // Drop the demux runtime.
    drop(r1);
    assert_eq!(Err(demux::ServiceStoppedError), r2.block_on(call).unwrap());
}

#[test]
fn test_arc_mutex() {
    let v = 5;
    let v2 = 10;
    let m = ArcMutex::new(v);
    // Loaded value should be the same as constructor argument.
    assert_eq!(v, *m.load());
    m.update(|x| {
        // Initial content of x should be the same as before update.
        assert_eq!(v, *x);
        // Concurrent load() should be possible and should return the value from before the update.
        assert_eq!(v, *m.load());
        *x = v2;
        assert_eq!(v, *m.load());
    });
    // After update, load() should return the new value.
    assert_eq!(v2, *m.load());
}
