# 🧾 SmartInventory

**SmartInventory** is a lightweight, web-based inventory management system designed for small retail businesses. It runs entirely in the browser using **Rust compiled to WebAssembly**, with a local **SQLite** backend for now. The system focuses on simple stock tracking, product management, and reporting — all without requiring a server or external API at this stage.

> 💡 Born out of real-world retail experience at GuardMart Supermarket, SmartInventory is designed to solve practical inventory challenges in a fast, offline-capable way.

---

## 📦 Features

### 🛍️ Product Management

- Add, edit, and delete products
- Assign categories, barcode, and supplier info
- Basic search and filter functionality

### 📊 Stock Tracking

- Real-time inventory quantity tracking
- View change history or movement logs
- Optional: Support for product-level notes or expiry dates

### 🔔 Reorder Alerts

- Set low-stock thresholds
- Simple dashboard alert when stock is low

### 📈 Basic Reporting

- View total product counts and stock value summaries
- Track product performance over time
- Optional: Export current stock data as CSV

---

## 🧪 Current Tech Stack

| Layer        | Technology                                     |
|--------------|------------------------------------------------|
| **Frontend** | Rust compiled to WebAssembly via `wasm-pack`   |
| **Database** | SQLite (local only for now)                    |
| **Serve Tool** | [`serve`](https://www.npmjs.com/package/serve) |
| **Styling**  | HTML and CSS (optional Tailwind)               |

---

## 🚀 Getting Started

### 1. Prerequisites

- Rust and Cargo installed
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/)
- Node.js and `serve` (Install with `npm install -g serve`)

### 2. Build the Project

```bash
wasm-pack build --target web
```

### 3. Serve the Application

```bash
serve -s ./pkg
```

---

Then open your browser to the local address provided (e.g., <http://localhost:5000>).

---

## ❗ Current Limitations

- No backend API or server logic implemented
- No user authentication or access control
- Not optimized for large-scale inventory data

---

## 🧭 Future Plans

- Implement user authentication and role-based access
- Add RESTful API or WebSocket support
- Enhance reporting with data visualization (charts, graphs)
- Add support for mobile and offline usage via PWA

---

## 👨‍💼 About the Creator

Jacob is a developer with practical experience managing inventory systems as a Line Manager at GuardMart Supermarket. He is using his real-world knowledge to build helpful, efficient tools using modern web technologies like Rust and WebAssembly.

---

## 📄 License

This project is licensed under the MIT License.
