# Portfolio — Rust + Leptos + TailwindCSS

A professional full-stack portfolio website built with:
- **Rust** — The backend and frontend logic
- **Leptos 0.7** — Full-stack reactive UI framework (SSR + WASM hydration)
- **Axum 0.7** — HTTP server
- **SQLite** via SQLx — Local database
- **TailwindCSS** — Styling
- **JWT** — Admin authentication
- **bcrypt** — Password hashing

## Features

### Public Portfolio
- Hero section with animated grid background
- About section with social links
- Work Experience timeline
- Featured + grid project layout
- Skills by category with proficiency levels
- Education history
- Certifications
- Contact section

### Admin Dashboard (`/admin`)
- Login with JWT auth (default: `admin` / `admin123`)
- Edit profile (name, title, bio, links, etc.)
- Add/Edit/Delete experience entries
- Add/Edit/Delete education entries
- Add/Edit/Delete projects (with featured flag)
- Add/Edit/Delete skills (with category + proficiency level)
- Add/Edit/Delete certifications
- Change password

---

## Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update

# Install cargo-leptos (the Leptos build tool)
cargo install cargo-leptos

# Install wasm target
rustup target add wasm32-unknown-unknown

# Install TailwindCSS CLI
npm install -g tailwindcss
# or with npx (no global install needed)

# Optional: install cargo-watch for hot reload
cargo install cargo-watch
```

---

## Running in Development

```bash
# 1. Clone and enter the project
cd portfolio

# 2. Copy env file
cp .env.example .env
# Edit .env if needed (DATABASE_URL defaults to sqlite:portfolio.db)

# 3. Start the dev server with hot reload
cargo leptos watch
```

The site will be available at `http://localhost:3000`

---

## Building for Production

```bash
# Build optimized release
cargo leptos build --release

# The output goes to ./target/release/portfolio (binary)
# and ./site/ (static assets)

# Run in production
./target/release/portfolio
```

---

## Project Structure

```
portfolio/
├── src/
│   ├── main.rs          # Binary entry point (SSR)
│   ├── lib.rs           # Library entry point (WASM)
│   ├── app.rs           # Root App component + routing
│   ├── models.rs        # Data models (Profile, Experience, etc.)
│   ├── state.rs         # Shared client state
│   ├── components/      # Reusable UI components
│   ├── pages/
│   │   ├── home.rs      # Public portfolio (all sections)
│   │   ├── login.rs     # Admin login page
│   │   └── admin.rs     # Admin dashboard
│   └── server/
│       ├── mod.rs       # Axum server runner
│       ├── db.rs        # SQLite database layer
│       ├── auth.rs      # JWT utilities
│       └── api.rs       # Server functions (API endpoints)
├── style/
│   └── input.css        # TailwindCSS entry
├── Cargo.toml
├── Leptos.toml          # Leptos config
├── tailwind.config.js
└── index.html           # HTML shell (Trunk template)
```

---

## Customizing

### First Login
1. Go to `http://localhost:3000/admin/login`
2. Log in with `admin` / `admin123`
3. Go to **Settings** and change your password immediately
4. Go to **Profile** and fill in your details

### Adding Content
All content is managed through the admin panel at `/admin`. No code changes needed.

### Changing the Theme
Edit `tailwind.config.js` and `style/input.css` for colors/fonts.

The color scheme uses:
- `zinc-950` — darkest background
- `zinc-900` — card backgrounds
- `zinc-800` — borders
- `emerald-400/500` — accent color

To change accent color, search-replace `emerald` with your Tailwind color of choice.

---

## Deployment

### With Docker (recommended)

```dockerfile
FROM rust:1.81 as builder
RUN cargo install cargo-leptos
RUN rustup target add wasm32-unknown-unknown
WORKDIR /app
COPY . .
RUN cargo leptos build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/portfolio .
COPY --from=builder /app/site ./site
ENV DATABASE_URL=sqlite:/data/portfolio.db
VOLUME /data
EXPOSE 3000
CMD ["./portfolio"]
```

### With a VPS (systemd service)

```ini
[Unit]
Description=Portfolio website
After=network.target

[Service]
Type=simple
WorkingDirectory=/opt/portfolio
ExecStart=/opt/portfolio/portfolio
Restart=always
Environment=DATABASE_URL=sqlite:/opt/portfolio/data/portfolio.db
Environment=LEPTOS_SITE_ADDR=0.0.0.0:3000

[Install]
WantedBy=multi-user.target
```

---

## Security Notes

- Change the default admin password (`admin123`) immediately
- Change `JWT_SECRET` in `.env` to a random string in production
- The JWT secret in `src/server/auth.rs` should come from your env in production
- Use HTTPS in production (via nginx reverse proxy or Cloudflare)
- The SQLite database file contains all your data — back it up regularly
