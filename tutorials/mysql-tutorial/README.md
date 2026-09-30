# Tutorial: Querying relational Databases

This folder contains the sample application written as part of the _Querying MySQL_ tutorial.

## Test locally with MySQL in Docker

Run MySQL in Docker:

```bash
docker run -d \
  --name mysql-container \
  -e MYSQL_USER=bob \
  -e MYSQL_PASSWORD=secret \
  -e MYSQL_DATABASE=sample \
  -e MYSQL_RANDOM_ROOT_PASSWORD=yes \
  -p 3306:3306 \
  mysql:latest
```

use `docker exec` to run the SQL scripts below against the database:

```bash
docker exec -i mysql-container mysql -ubob -psecret sample < seed.sql
```

Run the Spin application and pass required variables:

```bash
spin up --build \
  --variable mysql_host=localhost \
  --variable mysql_user=bob \
  --variable mysql_password=secret \
  --variable mysql_database=sample
```

## SQL Scripts

To provision the necessary `Products` table, use the following SQL command:

```sql
CREATE TABLE IF NOT EXISTS Products (
  Id varchar(36) PRIMARY KEY,
  Name TEXT NOT NULL,
  Price DOUBLE PRECISION
);
```

Use the following SQL commands to seed sample data:

```sql
INSERT INTO Products (Id, Name, Price)
SELECT 'faac630e-a645-4459-9d7e-751df4016a6e', 'V-Neck T-Shirt', 19.99
WHERE NOT EXISTS (SELECT Id FROM Products WHERE Id = 'faac630e-a645-4459-9d7e-751df4016a6e');

INSERT INTO Products (Id, Name, Price)
SELECT 'c01dce8a-3a50-4ef6-a0f1-7f9f48a238c8', 'Hoodie with Logo', 79.99
WHERE NOT EXISTS (SELECT Id FROM Products WHERE Id = 'c01dce8a-3a50-4ef6-a0f1-7f9f48a238c8');

INSERT INTO Products (Id, Name, Price) 
SELECT '6f062dc2-bbf2-4c6c-8169-3511462cd54b', 'Belt', 14.99
WHERE NOT EXISTS (SELECT Id FROM Products WHERE Id = '6f062dc2-bbf2-4c6c-8169-3511462cd54b');
```

## Building the Spin Application

Once you've cloned the repository, move into the tutorial folder ([./tutorials/mysql-tutorial](/tutorials/mysql-tutorial)) and run `spin build`:

```console
cd tutorials/mysql-tutorial
spin build
```

