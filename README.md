# blog-backend

![Uxío's list of posts](https://github.com/UxioAndrade/blog-backend/blob/main/uxio_blog.png?raw=true)

Backend for my personal page: [uxioandrade.com](http://uxioandrade.com)

## Setup

Note that you must be using nightly rust in order to run Rocket

**1- Create the DATABASE_URL environment variable**
```
DATABASE_URL=postgres://user:password@host/database
```

**2- Install the dependencies and build the app**
```bash
cargo build
```

**3- Run the database migrations**
```bash
diesel migration run
```

**4- Run the app in development mode**
```bash
cargo run
```

## Docker
Alternative, you may run the app in a Docker container by using the Dockerfiles defined in this respository.
