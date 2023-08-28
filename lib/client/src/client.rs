use basin_protocol::{publications, tx};
use capnp::message;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, FutureExt};
use std::net::ToSocketAddrs;

pub async fn push() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 3 {
        println!("usage: {} connect HOST:PORT", args[0]);
        return Ok(());
    }

    let addr = args[2]
        .to_socket_addrs()
        .unwrap()
        .next()
        .expect("could not parse address");

    tokio::task::LocalSet::new()
        .run_until(async move {
            let stream = tokio::net::TcpStream::connect(&addr).await?;

            println!("Connected to TCP Stream");

            stream.set_nodelay(true)?;
            let (reader, writer) =
                tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();

            // RPC code
            let rpc_network = Box::new(twoparty::VatNetwork::new(
                reader,
                writer,
                rpc_twoparty_capnp::Side::Client,
                Default::default(),
            ));
            let mut rpc_system = RpcSystem::new(rpc_network, None);
            let client: publications::Client =
                rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

            tokio::task::spawn_local(Box::pin(rpc_system.map(|_| ())));

            let mut request = client.push_request();

            let mut message = message::Builder::new_default();
            let mut builder = message.init_root::<tx::Builder>();

            builder.set_commit_l_s_n(1);
            let mut records = builder.reborrow().init_records(1);
            {
                let mut record = records.reborrow().get(0);
                record.set_action("foo");
            }
            request.get().set_tx(builder.into_reader())?;

            let mut message2 = message::Builder::new_default();
            let mut builder2 = message.init_root::<capnp::data::Builder>();
            // builder2.copy_from_slice()
            // request.get().set_signature(builder2.into_());

            let reply = request.send().promise.await.unwrap();

            // println!("Response: {}", reply.get().unwrap().get_response());

            Ok(())
        })
        .await
}

pub async fn create() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 3 {
        println!("usage: {} connect HOST:PORT", args[0]);
        return Ok(());
    }

    let addr = args[2]
        .to_socket_addrs()
        .unwrap()
        .next()
        .expect("could not parse address");

    tokio::task::LocalSet::new()
        .run_until(async move {
            let stream = tokio::net::TcpStream::connect(&addr).await?;

            println!("Connected to TCP Stream");

            stream.set_nodelay(true)?;
            let (reader, writer) =
                tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();

            // RPC code
            let rpc_network = Box::new(twoparty::VatNetwork::new(
                reader,
                writer,
                rpc_twoparty_capnp::Side::Client,
                Default::default(),
            ));
            let mut rpc_system = RpcSystem::new(rpc_network, None);
            let client: publications::Client =
                rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

            tokio::task::spawn_local(Box::pin(rpc_system.map(|_| ())));

            let mut request = client.create_request();

            // request.get().set_name("data");
            // request.get().set_schema("f168f6d01a154ab399dc0a36cb75ce9a");
            // request
            //     .get()
            //     .set_owner("0x31346Df523caE0a44eB9fb49E153D0e60E8016b6");
            // request
            //     .get()
            //     .set_columns("id bigserial, msg text not null, value numeric not null");

            request.send().promise.await.unwrap();

            Ok(())
        })
        .await
}
