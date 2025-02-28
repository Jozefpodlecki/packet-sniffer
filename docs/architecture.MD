
## **📦 Module Breakdown**

### **1️⃣ Web (Svelte)**
- **Purpose:** UI for packet analysis.
- **Tech Stack:** Svelte + TailwindCSS.
- **Communication:**
  - Uses **Tauri IPC** to interact with Rust backend.
  - Might use **WebSockets** for real-time updates.

### **2️⃣ App (Tauri Backend)**
- **Purpose:** Manages background tasks and UI communication.
- **Responsibilities:**
  - Runs a **background worker** to receive packets.
  - Calls `sniffer-consumer` for processing.
  - Serves the **filtered & structured data** to UI.

### **3️⃣ Sniffer-Consumer (Rust)**
- **Purpose:** Acts as a bridge between `app` and `sniffer`.
- **Responsibilities:**
  - Loads and interacts with `sniffer` (Rust FFI).
  - Filters and **parses raw packets into structured data**.
  - **Efficient packet forwarding** (e.g., via channels or WebSockets).

### **4️⃣ Sniffer (Rust, WinDivert)**
- **Purpose:** Core packet interception using **WinDivert**.
- **Responsibilities:**
  - Uses **raw sockets** to hook into network traffic.
  - Provides filtering (e.g., TCP, UDP, specific ports).
  - Exposes an **FFI API** (called by `sniffer-consumer`).

### **5️⃣ Client (Rust)**
- **Purpose:** Simulates a TCP-based **game client**.
- **Responsibilities:**
  - Connects to `server` over TCP.
  - Receives packets continuously.

### **6️⃣ Server (Rust)**
- **Purpose:** Simulates a TCP-based **game server**.
- **Responsibilities:**
  - Listens for `client` connections.
  - **Continuously sends packets** to simulate real-world traffic.

### **7️⃣ Shared (Rust Crate)**
- **Purpose:** Common models, serialization, and utilities.
- **Tech Stack:** Rust, `serde`, `bincode`.
- **Contents:**
  - `struct Packet { src_ip, dest_ip, payload, timestamp }`
  - `enum PacketType { TCP, UDP }`
  - Utility functions (e.g., `parse_packet`).

## **🔄 Communication Flow**
1️⃣ **Server → Client:** TCP server **sends packets** to the client.  
2️⃣ **Client → OS Stack:** Client receives packets, triggering OS network events.  
3️⃣ **Sniffer (WinDivert) Intercepts Packets.**  
4️⃣ **Sniffer-Consumer Processes Packets** and filters useful data.  
5️⃣ **App Receives Processed Packets** (via channels/WebSockets).  
6️⃣ **Svelte UI Displays Data** in real-time.  

