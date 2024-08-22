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