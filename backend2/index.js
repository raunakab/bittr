'use strict';
const Hapi = require('@hapi/hapi');
const Boom = require('@hapi/boom');

let uuid = 1;

const authenticateUser = (username, password) => {
    return upMap.some((obj) => obj.username === username && obj.password === password);
}
const createUuid = () => {
    let currentUuid = uuid;
    uuid += 1;
    return currentUuid;
}

const upMap = [
    {
        username: 'rbhagat',
        password: 'hapi',
    },
];
const activeSessions = [];

const init = async () => {
    const server = Hapi.server({
        port: 5000,
        host: 'localhost',
        routes: {
            cors: {
                origin: ['*'],
            },
        },
    });

    server.route({
        method: 'GET',
        path: '/',
        handler: (request, h) => {
            // return 'Hello World!';
            // console.log(request);
            let username = request.query.username;
            let password = request.query.password;

            let found = authenticateUser(username, password);
            if (found) {
                let uuid = createUuid();
                return uuid;
            } else {
                return Boom.unauthorized('invalid username/password');
            }
        }
    });

    await server.start();
    console.log('Server running on %s', server.info.uri);
};

process.on('unhandledRejection', (err) => {
    console.log(err);
    process.exit(1);
});

init();
