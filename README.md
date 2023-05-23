# Bonk

The Bonking App for Bonks

To run:

```
docker compose up
```

This will start up red panda and the kafka cluster

```
cargo run --bin consumer
```

This will set up the consumer, it will listen on topic: test

```
cargo run --bin producer
```

This will set up a producer of bonk messages, it will send things to topic: test