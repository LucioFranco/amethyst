#[cfg(test)]
mod test {
    extern crate fern;

    use amethyst_core::shred::{DispatcherBuilder, SystemData};
    use amethyst_core::specs::{Builder, Join, World, WriteStorage};
    use log::LevelFilter;
    use std::io;
    use std::net::{IpAddr, SocketAddr};
    use std::str::FromStr;
    use std::thread::sleep;
    use std::time::Duration;

    use self::fern::Dispatch;

    use super::super::NetSocketSystem;
    use super::super::utils::*;
    use super::super::net::*;
    use super::super::connection::*;

    #[test]
    fn single_packet_early() {
        let mut world_client = World::new();
        let mut world_server = World::new();

        let mut client_dispatch = DispatcherBuilder::new()
            .with(
                NetSocketSystem::<()>::new("127.0.0.1", 21201, Vec::new()).unwrap(),
                "s",
                &[],
            ).build();

        client_dispatch.setup(&mut world_client.res);

        let mut server_dispatch = DispatcherBuilder::new()
            .with(
                NetSocketSystem::<()>::new("127.0.0.1", 21200, Vec::new()).unwrap(),
                "s",
                &[],
            ).build();

        server_dispatch.setup(&mut world_server.res);

        let mut conn_to_server = NetConnection::<()>::new(SocketAddr::new(
            IpAddr::from_str("127.0.0.1").unwrap(),
            21200,
        ));

        let mut conn_to_client = NetConnection::<()>::new(SocketAddr::new(
            IpAddr::from_str("127.0.0.1").unwrap(),
            21201,
        ));

        let test_event = NetEvent::TextMessage {
            msg: "1".to_string(),
        };

        conn_to_server.send_buffer.single_write(test_event.clone());
        world_client.create_entity().with(conn_to_server).build();

        let mut rcv = conn_to_client.receive_buffer.register_reader();
        let conn_to_client_entity = world_server.create_entity().with(conn_to_client).build();

        client_dispatch.dispatch(&mut world_client.res);
        sleep(Duration::from_millis(500));
        server_dispatch.dispatch(&mut world_server.res);

        let storage = world_server.read_storage::<NetConnection<()>>();
        let comp = storage.get(conn_to_client_entity).unwrap();


        assert_eq!(comp.receive_buffer.read(&mut rcv).next(), Some(&test_event));
        // We should have consumed the only event in the iterator by calling next().
        assert!(comp.receive_buffer.read(&mut rcv).count() == 0);
    }
    #[test]
    fn send_receive_100k_packets() {
        Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{}][{}] {}",
                    record.target(),
                    record.level(),
                    message
                ))
            }).level(LevelFilter::Debug)
            .chain(io::stdout())
            .apply()
            .unwrap_or_else(|_| {
                debug!("Global logger already set, default amethyst logger will not be used")
            });

        let mut world_client = World::new();
        let mut world_server = World::new();

        let mut client_dispatch = DispatcherBuilder::new()
            .with(
                NetSocketSystem::<()>::new("127.0.0.1", 21204, Vec::new()).unwrap(),
                "s",
                &[],
            ).build();

        client_dispatch.setup(&mut world_client.res);
        let mut server_dispatch = DispatcherBuilder::new()
            .with(
                NetSocketSystem::<()>::new("127.0.0.1", 21205, Vec::new()).unwrap(),
                "s",
                &[],
            ).build();

        server_dispatch.setup(&mut world_server.res);

        let conn_to_server = NetConnection::<()>::new(SocketAddr::new(
            IpAddr::from_str("127.0.0.1").unwrap(),
            21205,
        ));
        let mut conn_to_client = NetConnection::<()>::new(SocketAddr::new(
            IpAddr::from_str("127.0.0.1").unwrap(),
            21204,
        ));

        world_client.create_entity().with(conn_to_server).build();

        let mut rcv = conn_to_client.receive_buffer.register_reader();
        let conn_to_client_entity = world_server.create_entity().with(conn_to_client).build();


        for _i in 0..10 {
            sleep(Duration::from_millis(50));
            {
                let mut sto = WriteStorage::<NetConnection<()>>::fetch(&world_client.res);
                for mut cmp in (&mut sto).join() {
                    for _i in 0..10000 {
                        let test_event = NetEvent::TextMessage {
                            msg: format!("Test {}", _i),
                        };
                        cmp.send_buffer.single_write(test_event.clone());
                    }
                }
            }

            client_dispatch.dispatch(&mut world_client.res);
            sleep(Duration::from_millis(500));
            server_dispatch.dispatch(&mut world_server.res);
            let storage = world_server.read_storage::<NetConnection<()>>();
            let comp = storage.get(conn_to_client_entity).unwrap();
            assert_eq!(comp.receive_buffer.read(&mut rcv).count(), 10000);
        }
    }
}
