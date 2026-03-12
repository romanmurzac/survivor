# Survivor

[![Built with Rust](https://img.shields.io/badge/built%20with-rust-blue)](https://github.com/rust-lang/rust)
[![Built with egui](https://img.shields.io/badge/built%20with-egui-blue)](https://github.com/emilk/egui)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)\
![Version](https://img.shields.io/badge/version-0.1.0-orange)
[![Build Status](https://github.com/romanmurzac/survivor/actions/workflows/deploy.yml/badge.svg)](https://github.com/romanmurzac/survivor/actions)


**Survivor** is a financial simulation tool that helps you understand how long you can sustain your lifestyle based on your **savings, income, expenses, loans, and future scenarios**.

The application simulates your financial situation month by month and shows whether you:

* stay financially stable
* survive long-term
* or go bankrupt

It is designed to help you **experiment with financial decisions before making them in real life.**

---

# What the App Does

Survivor simulates a **cashflow timeline** and generates a monthly financial report based on:

* savings
* incomes
* expenses
* loans
* financial scenarios

Each month the engine calculates:

Savings = Previous Savings + Incomes − Expenses − Loans

The simulation continues until the defined **time horizon**.

---

# Key Features

## Financial Simulation

Simulate your financial future over months or years.

You can adjust:

* starting savings
* financial horizon
* target savings
* income streams
* expenses
* loans

---

## Income Management

Add multiple income sources.

Each income has:

* name
* amount
* type
* category

Types:

* **Recurrent** – monthly income
* **One Time** – income that happens in a specific month

Categories:

* **Active** – salary, consulting, freelance
* **Passive** – investments, dividends, rentals

---

## Expense Management

Expenses can be categorized as:

* **Fixed** – constant monthly expenses
* **Variable** – flexible spending
* **Unpredictable** – occurs in a specific month

Examples:

Fixed

* rent
* insurance
* subscriptions

Variable

* groceries
* entertainment

Unpredictable

* medical bills
* repairs
* emergencies

---

## Loan Tracking

Loans include:

* name
* remaining balance
* monthly payment
* interest

This allows realistic modeling of financial obligations.

---

## Scenario Simulation

Scenarios simulate **future financial events**.

Examples:

* salary increase
* salary loss
* expense cuts
* expense growth

Rules can target:

* active income
* passive income
* fixed expenses
* variable expenses
* unpredictable expenses

Actions include:

* loss
* increase
* cut

This allows testing **"what if" situations**.

---

# Status Results

At the end of the simulation the app calculates your **financial status**.

Possible results:

**Survivor**

Your finances remain stable over time.

**In The Game**

Your finances remain positive but unstable.

**Still Alive**

Your savings decrease but you remain solvent.

**Bankrupt**

Your savings become negative.

Example message:

```
In 6 months you become BANKRUPT
```

or

```
You are a SURVIVOR from the first month
```

---

# Visualization

The app includes:

## Cashflow Chart

A bar chart that shows monthly savings evolution.

Positive savings appear as **green bars**.
Negative savings appear as **red bars**.

Axis:

* X axis → Month
* Y axis → Savings

---

## Financial Report

A full monthly table:

| Month | Savings | Incomes | Expenses | Loans |
| ----- | ------- | ------- | -------- | ----- |

You can export the report to **CSV**.

---

# CSV Export

The report can be downloaded as a CSV file.

File format:

```
Month,Savings,Incomes,Expenses,Loans
1,4500,3000,2000,500
2,5000,3000,2000,500
```

The filename includes a timestamp:

```
survivor_report_2026-03-11_15-42-30.csv
```

---

# How to Use the App

1. Set **starting savings**
2. Set **time horizon**
3. Set **target savings**
4. Add incomes
5. Add expenses
6. Add loans
7. Optionally add scenarios
8. Review the chart and report

The simulation updates **in real time**.

---

# Running Locally

## Desktop

Run:

```
cargo run
```

---

## Web Version

Install dependencies:

```
cargo install trunk
rustup target add wasm32-unknown-unknown
```

Run the web server:

```
trunk serve
```

Open:

```
http://127.0.0.1:8080
```

---

# Project Structure

```
src/
 ├── domain/
 │   ├── income.rs
 │   ├── expense.rs
 │   ├── loan.rs
 │   └── scenario.rs
 │
 ├── engine/
 │   └── simulation.rs
 │
 ├── gui/
 │   ├── app.rs
 │   ├── chart_view.rs
 │   ├── report_view.rs
 │   └── panels/
 │
 ├── report/
 │   └── report.rs
 │
 └── utils/
     └── export.rs
```

---

# Technology

The application is built using:

Rust
egui
eframe
egui_plot
WebAssembly (WASM)

This allows the app to run:

* as a **desktop application**
* directly in the **browser**

---

# Future Improvements

Planned features:

* mobile optimized UI
* scenario templates
* probability-based expenses
* financial goal planning
* FIRE simulation
* Monte Carlo simulations
* multi-currency support

---

# Version

Current version:

**v0.1.0**

---

# Disclaimer

This tool is for **educational and planning purposes only**.

It does not provide financial advice.

---

# Author

Created with **Rust**.
