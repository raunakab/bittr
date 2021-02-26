# Luo
A group ledger application.

### Setup
1. start the database server

        pg_ctl -D ${DB_HOME} start

3. initialize diesel

        diesel setup

3. start the frontend

        cd frontend && npm start

4. start the backend (using cargo watch)

        cd backend && cargo watch -x run

### Shutdown
1. shutdown frontend process
2. shutdown backend process
3. shutdown databse server

        pg_ctl -D ${DB_HOME} stop

### Run/Redo migrations
* run:

        diesel migration run

* redo:

        diesel migration redo

### Useful CURL Commands:
* put/post:

        curl -X PUT -H "Content-Type: application/json" -d '{"username":"testUser", "passwd":"testPasswd"}' localhost:5000/users
        curl -X POST -H "Content-Type: application/json" -d '{"new_username":"testUser2", "new_passwd":"testPasswd2"}' localhost:5000/users/testUser

* get:

        curl -X GET localhost:5000/users/testUser2
