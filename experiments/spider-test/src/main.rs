use async_std::{
    task,
    sync::{Arc, RwLock, channel, Mutex, Sender},
};
use std::{
    collections::{
        HashMap,
        HashSet,
    },
    sync::atomic::{
        AtomicUsize,
        Ordering::SeqCst,
    },
    time::Duration,
};
use rand::{self, Rng};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let lock = Arc::new(RwLock::new(()));
    let lock2 = lock.clone();

    task::block_on(async {
        // demo1().await?;
        demo2().await?;
        println!("Done!");
        Ok(())
    })
}

async fn demo1() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let uri = "http://localhost:8000/a.toml";
    let mut resp = dbg!(surf::get(uri).await?);
    let dat_bod = dbg!(resp.body_string().await?);
    dbg!(str_to_uris(&dat_bod));
    Ok(())
}

struct Recur<T> {
    data: T,
    next: Sender<Recur<T>>,
}

async fn demo2() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let (tx_outer, rx) = channel::<Recur<String>>(1024);
    const MAX_IN_FLIGHT: usize = 256;
    let (txs, rxs) = channel::<()>(MAX_IN_FLIGHT);
    tx_outer.send(
        Recur {
            data: "http://localhost:8000/a.toml".to_string(),
            next: tx_outer.clone()
        }
    ).await;
    drop(tx_outer);

    // Fill the counting semaphore
    for _ in 0..MAX_IN_FLIGHT {
        txs.send(()).await;
    }

    let ctr = AtomicUsize::new(0);
    let seen = Arc::new(Mutex::new(HashSet::new()));

    while let Some(rec) = rx.recv().await {
        let (next, tx) = (rec.data, rec.next);

        {
            let mut g = seen.lock().await;
            if !g.insert(next.clone()) {
                continue;
            }
        }


        // take a counting semaphore
        let _ = rxs.recv().await;

        let new = ctr.fetch_add(1, SeqCst);

        // Get the body
        // split the body to uris
        // push each into the channel
        let txs = txs.clone();

        task::spawn(async move {
            let (next, _) = dbg!(next, new);

            let new_uris = str_to_uris(
                &surf::get(&next).await.unwrap()
                .body_string().await.unwrap()
            );
            // Simulate a slow response
            // let sleep_ms = 250 + (rand::random::<u64>() % 1000);
            // let sleep_ms = 1000;
            // task::sleep(Duration::from_millis(sleep_ms)).await;

            // Replace the counting semaphore
            txs.send(()).await;

            for uri in new_uris {
                tx.send(Recur {
                    data: uri,
                    next: tx.clone()
                }).await;
            }
        });
    }

    Ok(())
}

fn str_to_uris(body: &str) -> Vec<String> {
    body.lines().filter_map(|l| {
        if l.ends_with(".toml") {
            Some(format!("http://localhost:8000/{}", l))
        } else {
            None
        }
    }).collect()
}
