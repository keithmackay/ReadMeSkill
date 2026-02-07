# task-api

REST API for managing tasks, built with Express and backed by PostgreSQL. Provides a standard CRUD interface with JWT authentication, request logging, and security hardening out of the box.

## Highlights

- **JWT authentication** — Secure endpoints with token-based auth
- **PostgreSQL storage** — Persistent task data with the pg driver
- **Security hardened** — Helmet middleware for HTTP header protection
- **CORS enabled** — Cross-origin requests supported by default
- **Request logging** — Morgan middleware logs all requests
- **Health check** — `GET /health` endpoint for monitoring

## Getting Started

### Prerequisites

- Node.js 20+
- PostgreSQL 15+

### Installation

```bash
git clone <repo-url>
cd task-api
npm install
cp .env.example .env
# Edit .env with your database credentials
```

## Usage

Start the development server:

```bash
npm run dev
```

The API runs on `http://localhost:3000` by default.

### Endpoints

```bash
# List all tasks
curl http://localhost:3000/api/tasks

# Create a task
curl -X POST http://localhost:3000/api/tasks \
  -H "Content-Type: application/json" \
  -d '{"title": "Buy groceries", "done": false}'

# Get a specific task
curl http://localhost:3000/api/tasks/1

# Update a task
curl -X PUT http://localhost:3000/api/tasks/1 \
  -H "Content-Type: application/json" \
  -d '{"title": "Buy groceries", "done": true}'

# Delete a task
curl -X DELETE http://localhost:3000/api/tasks/1
```

## Configuration

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `PORT` | Server port | `3000` | No |
| `DATABASE_URL` | PostgreSQL connection string | — | Yes |
| `NODE_ENV` | Environment (`development`, `production`) | `development` | No |
| `JWT_SECRET` | Secret key for JWT signing | — | Yes |
| `LOG_LEVEL` | Logging verbosity | `debug` | No |

## Development

```bash
git clone <repo-url>
cd task-api
npm install
npm test
```

| Command | Description |
|---------|-------------|
| `npm run dev` | Start development server with auto-reload |
| `npm test` | Run tests with coverage |
| `npm run lint` | Lint source files |
| `npm start` | Start production server |

## Contributing

Contributions are welcome. Fork the repo, create a feature branch, and open a pull request.

## License

[MIT](LICENSE)
