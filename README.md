# Luo
A group ledger application.

### Setup
1. start the database server
    pg_ctl -D ${DB_HOME} start
2. initialize diesel
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
