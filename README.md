# [Survivor](https://romanmurzac.github.io/survivor/instructions.html)

[![Built with Rust](https://img.shields.io/badge/built%20with-rust-blue)](https://github.com/rust-lang/rust)
[![Built with egui](https://img.shields.io/badge/built%20with-egui-blue)](https://github.com/emilk/egui)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)\
![Version](https://img.shields.io/badge/version-0.1.2-orange)
[![Build Status](https://github.com/romanmurzac/survivor/actions/workflows/deploy.yml/badge.svg)](https://github.com/romanmurzac/survivor/actions)

[survivor](https://romanmurzac.github.io/survivor/instructions.html)

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
* incomes
* expenses
* loans

---

## Income Management

Income compute all your existing incomes.\
Add multiple income sources.

Attributes:
* name
* amount
* type
* category

Types:
* **Recurrent** – monthly income
* **OneTime** – income that happens once in a specific month

Categories:
* **Active** – earned from directly participating in work
* **Passive** – earned with minimal daily effort

Examples:
* **Recurrent + Passive** - investments, dividends, rentals
* **OneTime + Active** - side project, consulting, commissions
* **OneTime + Passive** - intelectual property, patents, sell property

---

## Expense Management

Expense compute all your existing expenses.\
Add multiple expense sources.

Attributes:
* name
* amount
* type

Types:
* **Fix** – constant monthly expenses
* **Variable** – flexible spending, aproximate to occur every 6 months
* **Unpredictable** – occurs once in a specific month

Examples:
* **Fix** - rent, groceries, subscriptions
* **Variable** - shopping, entertainment, vacation
* **Unpredictable** - medical bills, repairs, emergencies

---

## Loan Management

Loan compute all your existing loans.\
Add multiple loan sources.

Attributes:
* name
* remaining balance
* monthly payment
* interest

Examples:
* **Mortgage** - mortgage, 350.000, 1.100, 1.100
* **Leasing** - leasing, 170.000, 550, 300
* **Student Loan** - student, 60.000, 300, 100

---

## Scenario Simulation

Scenarios simulate **future financial events**.\
This allows testing **"what if" situations**.\
Add multiple financial scenarios.

Attributes:
* start month
* target
* action

Targets:
* **ActiveIncome** – earned from directly participating in work
* **PassiveIncome** – earned with minimal daily effort
* **FixExpense** – constant monthly expenses
* **VariableExpense** – flexible spending, aproximate to occur every 6 months
* **UnpredictableExpense** – occurs once in a specific month

Actions:
* **Loss** - Reduce the scenario's target to zero starting from specified start month
* **Increase** - Increase the scenario's target with specific percent starting from specified month
* **Cut** - Decrease the scenario's target with specific percent starting from specified month

Examples:
* **Loss + ActiveIncome** - initial 100, after 0
* **Increase + FixExpense (with 10%)** - initial 100, after 110
* **Cut + VariableExpense (with 10%)** - initial 100, after 90

---

# Status Results

At the end of the simulation the app calculates your **financial status**.

Results:

**Survivor**\
You reached financial freedom within the selected time horizon.

**InTheGame**\
Your finances remain positive throughout the simulation.

**StillAlive**\
Your savings are declining, but you remain financially solvent.

**Bankrupt**\
Your savings become negative during the simulation.

Example message:

```
In 20 months you become SURVIVOR.
```

or

```
You are InTheGame.
```

or

```
You are StillAlive.
```

or

```
In 6 months you become BANKRUPT.
```

---

# Visualization

The app includes:
* Chart
* Report

## Cashflow Chart

A bar chart that shows monthly savings evolution.

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
9. Export CSV Report for analysis

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
trunk serve --public-url /survivor/
```

Open:

```
http://127.0.0.1:8080
```

Create release version:

```
trunk build --release --public-url /survivor/
```

---

# Project Structure

```
src/
 ├── domain/
 │   ├── expense.rs              # expense cash flow direction and frequency
 │   ├── income.rs               # income cash flow direction and frequency
 │   ├── loan.rs                 # stateful debt tracking
 │   ├── scenario.rs             # rules that modify financial data over time
 │   ├── status.rs               # the resulting state of a simulation run
 │   └── transaction.rs          # a unified wrapper for processing all financial items
 │
 ├── engine/
 │   └── simulation.rs           # the primary simulation runner and its associated configuration
 │
 ├── gui/
 │   ├── editors/
 │   │   ├── expenses_editor.rs  # UI components for managing recurring and unpredictable expenses
 │   │   ├── incomes_editor.rs   # UI components for managing active and passive income streams
 │   │   ├── loans_editor.rs     # UI components for tracking and adjusting debt amortization schedules
 │   │   └── scenarios_editor.rs # UI components for defining time-based financial interventions and rules
 │   │
 │   ├── panels/
 │   │   ├── controls_panel.rs   # the main dashboard for configuring simulation variables and managing data
 │   │   └── results_panel.rs    # the display area for viewing simulation outcomes, charts, and reports
 │   │
 │   ├── widgets/
 │   │   ├── chart_view.rs       # visualizer component for rendering financial performance charts
 │   │   └── report_view.rs      # tabular component for rendering detailed simulation reports
 │   └── app.rs                  # the central state container and `eframe` application logic
 │
 ├── report/
 │   └── report.rs               # the snapshot structure representing a single month's financial status
 │
 ├── utils/
 │   ├── export.rs               # provides functionality for serializing and downloading data as CSV
 │   └── preview.rs              # provides console-based debugging utilities for rapid data inspection
 └── main.rs                     # application entry point
```

---

# Technology

The application is built using:

Rust\
egui\
eframe\
egui_plot\
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
* FIRE (Financial Independence, Retire Early) simulation
* Avalanche and Snowball
* Monte Carlo simulations
* multi-currency support

---

# Disclaimer

This tool is for **educational and planning purposes only**.\
It does not provide financial advice.


