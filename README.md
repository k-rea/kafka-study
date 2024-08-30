# Apache Kafkaの勉強
参考:ApacheKafkaをはじめる-1.0.0

## 2章
### Docker コンテナの起動と確認
#### コンテナの起動
```shell
docker compose up -d
```

#### コンテナ状態の確認
4つのコンテナが起動中であることを確認
```shell
docker compose ps
```

#### GUI・CLIの確認
- `http://localhost:9000`に接続できるか確認
- `docker exec -it cli bash`でCLIに接続できるか確認

### Topicの作成
cliに接続後、

ird-first-topicを作成
```shell
kafka-topics --bootstrap-server broker:9092 --create --topic ird-first-topic --partitions 3 --replication-factor 1
```

作成されたtopicを確認
```shell
kafka-topics --bootstrap-server broker:9092 --list 
kafka-topics --bootstrap-server broker:9092 --describe --topic ird-first-topic
```

### イベントの送信
下記実行後、テキストを送信
```shell
kafka-console-producer --bootstrap-server broker:9092 --topic ird-first-topic
```

### イベントの受信
別のターミナルで
```shell
kafka-console-consumer --bootstrap-server broker:9092 --topic ird-first-topic --group G1 --from-beginning
```

### consumerグループの確認
```shell
kafka-consumer-groups --bootstrap-server broker:9092 --describe --group G1
```

## Producerの実装
project rootでcliにログイン
```shell
docker compose exec cli bash
```

```shell
kafka-console-consumer --bootstrap-server broker:9092 --topic ticket-order --group G1 --from-beginning
```

別ターミナルで
```shell
cd basic-producer
RUST_LOG=info cargo run
```