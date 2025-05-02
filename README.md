# 🌱 DeGarden: Decentralized IoT Garden Management on Solana

DeGarden is a decentralized protocol built on the Solana blockchain using the Anchor framework. It enables secure and trustless interaction between IoT devices like moisture sensors and flowmeters, hosts, and users in a decentralized ecosystem. Users can deposit collateral, register sensors, and perform token-based transactions, all while maintaining transparency and accountability through on-chain logic.

---

## 📁 Project Structure

- `constants`: Defines global constants used across the program.
- `error`: Custom error definitions for precise failure handling.
- `events`: Events emitted by program instructions.
- `instructions`: Core logic for handling user and system instructions.
- `state`: Defines all on-chain data structures.

---

## 🚀 Features

### 🏗️ Initialization
- `initialize_global_state`: Initializes global settings like the token price.

### 👤 Host Management
- `add_host`: Allows new hosts to be added to the system.

### 📡 Sensor Registration
- `register_moisture_sensor`: Registers a new moisture sensor with GPS coordinates.
- `register_flowmeter_sensor`: Registers a flowmeter sensor with location data.

### 💰 Collateral and Tokens
- `deposit_collateral`: Users deposit tokens as collateral for sensors.
- `withdraw_collateral`: Withdraw collateral when conditions are met.
- `buy_tokens`: Purchase ecosystem tokens.
- `sell_tokens`: Redeem tokens for lamports.

### 📊 Sensor Data & Payments
- `pay_sensor_data`: Pay for sensor data usage.
- `slash_collateral`: Penalize misbehaving sensors/hosts by slashing collateral.

---

## 🧠 Design Highlights

- **Modular Architecture:** Separated logic for easy maintenance and testing.
- **Event Logging:** Each major action emits events for traceability.
- **Security:** Custom errors and type-safe instructions reduce vulnerabilities.
- **Token Economy:** Implements a token-based access and incentive model for IoT data.

---

## 📜 Deployment

This program is deployed to Devnet. It uses the following program ID: J8vn4oXKvsJyyRPcEscXcPkdpcEz4EoPhjM7ebVcvhqi

