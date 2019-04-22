# AkiraServerV3
( Experimental )
## WebServer Project
Reimplementacion de servidor web por 3º vez , esta vez en rust!
### ToDo
* Functional base
    * Enviar archivos solicitados
        * Chunkear los archivos
* Config
    * Puerto
    * Listado de archivos
    * CORS
    * 404 Por defecto
    * Rewrites ?
* Paginas auto generadas  
    *Directory listing
* Headers  
    *Content Type  
    *Content Size  
### Hecho
* Threadpool para requests
* Configuraciones desde archivo
### Mejoras Pendientes  
* Http Request Client -> Valor sin cabecera
* Escuchar cambios en el archivo de configuracion
### Bug

* /Sleep


## Docs

### Tecnologias

#### Rust

[Rust](https://www.rust-lang.org/)  
[Awesome Rust](https://github.com/rust-unofficial/awesome-rust)

#### Protocolo http

[Mozilla MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Overview)


### Librerias Usadas

#### Sample Name

### Funcionamiento
Se basa en las librerias:
* std::net::TcpListener;
* std::net::TcpStream;  

Con ellas creamos un servicio que escucha en una direccion y puerto determinados, 
cuando se recive una conexion, 
esta se pasa a un hilo secundario para que el programa pueda seguir atendiendo peticiones.  

En este, lo primero que se realiza es un parseo de la request http obteniendo 2 structs,
los cuales nos proporcionaran toda la informacion necesaria para procesar la peticion.

Request:

```rust
#[derive(Debug, Default)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub client: Client,
}
```

WIP ↓
```rust
#[derive(Debug, Default)]
pub struct Client {
    pub version: String,
    pub browser: String,
    pub connection: String,
    pub cache_control: String,
    pub upgrade_insecure_requests: String,
    pub user_agent: String,
    pub dnt: String,
    pub accept: String,
    pub accept_encoding: String,
    pub accept_anguage: String,
    pub cookie: String,
    pub other: String,
}
```
