# Introdução

Este software recebe uma chamada HTTP, com um token para fazer a atualização de uma imagem no docker.

## Requisitos

### Desenvolvimento

1. Deixe um terminal rodando com o proxy entre `/var/run/docker.sock` e `localhost:8080`

```bash
apt install socat
socat TCP-LISTEN:8080,bind=127.0.0.1,reuseaddr,fork,range=127.0.0.0/8 UNIX-CLIENT:/var/run/docker.sock
```

2. Execute o `cargo run`
