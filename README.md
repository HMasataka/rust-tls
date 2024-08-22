# golang-tls

## 証明書の発行

- 秘密鍵

```bash
openssl genrsa 2048 > credentials/develop/server.key
```

- 証明書署名要求書

```bash
openssl req -new -key credentials/develop/server.key > credentials/develop/server.csr
```

- サーバ証明書

```bash
openssl x509 -days 3650 -req -signkey credentials/develop/server.key < credentials/develop/server.csr > credentials/develop/server.crt
```
