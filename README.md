# Build Tools & Versions Used

## Dependencies in Local Machine

- Rust/Cargo

## Libs "Third Party"

- Chrono (for parsing timestamps)
- Reqwest (Get nodes info in JSON string)
- Rusqlite (Is a nice approach for this project, I will explain more about this below.)
- Serde (Serializer/Deserializer JSON objects use structs)
- Serde_json (Response with a JSON with a good formatting)

# Steps to Run the App

## Run the Following Commands

```bash
cargo run --manifest-path ./challenge/Cargo.toml
```

This will build and run the program on your machine.

The port 8080 should be used by the program; verify if this port is free or change it in `./challenge/constants.rs`

## For Running Tests

```bash
cargo test --manifest-path ./challenge/Cargo.toml
```

# What Was the Reason for Your Focus? What Problems Were You Trying to Solve?

## Current Problem Proposed by Challenge

- Get data from an external endpoint and save it in the database.
- Retrieve data from the database and serve it in an endpoint formatted in JSON.

## How My Solution Solves This

Look at this diagram.
![Diagram](https://i.postimg.cc/XqSr2057/Untitled-2025-03-25-0938-2.png)

As you can see, my solution consists of two things:

- Retrieve data from the API of mempool.
- Serve the data for user use with a cache system.

### About Retrieving Data from API

The API response is in JSON, which is a good format for communication between two distinct programs, but not good for storage. JSON casts must store and become hard to retrieve data from the database, depending on how it is implemented. To solve this, parse the JSON and store the required data into a struct and save it in a database.

This process is periodic and does not have any relation with retrieving data by the user.

### About Server Data and Cache System

Maybe you're thinking... Why cache?

Why not retrieve the data from the database without cache?

It is a good question, and I have a good answer.

Think—without a cache system, every time the user requests a query, your database will need to retrieve the data for the user. This could be slow (and another issue I will address more below). With a cache system, the user does not need to request a new query to retrieve data because this data is in the cache. The system retrieves the data for the user much faster and prevents unnecessary queries to the database.

A query to the database is only made under two conditions:

- A user requests the data.
- The "flag" in the cache becomes outdated.

Okay, good—but isn't it simpler to make the cache query every time the database updates?

Hmm... Maybe, but think about this—users request data less frequently during specific times, such as nighttime. If there are no user requests for hours, the cache will make a query to the database every time it updates, even though no users need the data. It is better to request a new query only when there is a user request and outdated cache.

## Why My Solution Focuses on Reducing Database Usage

I have a good answer for this.

Think—if returning the data to the user without querying the database is faster than making a query, it reduces computational costs. Depending on the implementation, this project will lower infrastructure costs since there will be fewer queries to process.

# How Long Did You Spend on This Project?

I have spent a lot of time on this project this week. I have been working every day since I received the challenge—approximately 20 to 30 hours.

# Did You Make Any Trade-offs for This Project? What Would You Have Done Differently with More Time?

## About Trade-offs

Yeah, the cache system is good, but it requires more processing power and uses Mutex, which I will explain more about below. I used Reqwest and Serde_json to make the code cleaner and more practical, but with more time, it is possible to implement a custom solution without using third-party libs.

### About the Rusqlite

Rusqlite may not be the most popular choice for large web projects, but for this project, it sounds good to me. There are no massive tables to store, no need for complex configurations for database actions (such as scheduling stored procedures), and Rusqlite does not require binding a port or setting up a container. For this project, it is a good choice. However, this depends on the challenge requirements. If I need to create more tables for users or store sensitive data, Rusqlite may not be the best choice.

## If I Had More Time

- Consider using async/await with Tokio.
- Review the error handling.
- Add more tests—unit, integration, and stress tests.
- Try to reduce Mutex usage.
- Add a queue system for requests (to ensure all requests are responded to without missing any).

# What Do You Think Is the Weakest Part of Your Project?

### Mutex

Mutex can cause a panic if not handled correctly.

### Error Handling

I want to spend more time creating a robust error handler for this project. Right now, it has a verbose error handler that is not the most efficient.

### Tests

This project does not have many tests.

### Dependence on the Mempool.Space API REST

If Mempool is down, there is no alternative source to feed the database, which could be problematic.

### Request Handling

The cache system will protect the database from excessive direct queries, but an attacker can overload the server with many requests, blocking responses to other users. This can be fixed with a queue system.
