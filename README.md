# API con Rust, GraphQL y MongoDB

Esta es una API desarrollada en Rust que sigue el patrón de diseño de Puertos y Adaptadores. Utiliza GraphQL para la consulta de datos y MongoDB como base de datos.

## Características

- Patrón de diseño de Puertos y Adaptadores
- API GraphQL
- Conexión a MongoDB
- Pruebas unitarias
- Configuración de entorno con `.env`
- Dockerfile para un despliegue sencillo

## Módulos

La API cuenta con los siguientes módulos:

- Reportes
- Padres
- Personal
- Finanzas
- Blogs

## Requisitos

- Rust
- Docker (opcional)
- MongoDB

## Configuración

1. Clona el repositorio:
   ```bash
   git clone https://github.com/tu-usuario/tu-repositorio.git
   ```

2. Crea un archivo `.env` a partir del ejemplo:
   ```bash
   cp .env.example .env
   ```

3. Modifica el archivo `.env` con tus propias credenciales de MongoDB.

## Ejecución local

1. Inicia la aplicación:
   ```bash
   cargo run
   ```

2. La API estará disponible en `http://localhost:8080`.

## Ejecución con Docker

1. Construye la imagen de Docker:
   ```bash
   docker build -t mi-api-rust .
   ```

2. Ejecuta el contenedor:
   ```bash
   docker run -p 8080:8080 mi-api-rust
   ```

## Endpoints

- **GraphQL:** `http://localhost:8080/graphql`
- **REST (Health Check):** `http://localhost:8080/health`