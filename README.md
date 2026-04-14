<p align="center">
  <img src="https://img.shields.io/badge/Tauri-2.0-FFC131?style=flat-square&logo=tauri&logoColor=white" />
  <img src="https://img.shields.io/badge/Rust-1.75+-000000?style=flat-square&logo=rust&logoColor=white" />
  <img src="https://img.shields.io/badge/React-19-61DAFB?style=flat-square&logo=react&logoColor=black" />
  <img src="https://img.shields.io/badge/TypeScript-5-3178C6?style=flat-square&logo=typescript&logoColor=white" />
  <img src="https://img.shields.io/badge/Vite-6-646CFF?style=flat-square&logo=vite&logoColor=white" />
  <img src="https://img.shields.io/badge/Cargo-Build-8B4513?style=flat-square&logo=rust&logoColor=white" />
  <img src="https://img.shields.io/badge/Node.js-20+-339933?style=flat-square&logo=nodedotjs&logoColor=white" />
</p>

# System-Monitor Dashboard

## 📖 Descrizione Dettagliata

Il System-Monitor Dashboard è un'applicazione cross-platform di monitoraggio delle risorse di sistema progettata per fornire una visione granulare, in tempo reale e altamente performante dello stato hardware e software di un dispositivo computing. Non si tratta di un semplice tool di diagnostica, ma di un pannello di controllo avanzato (dashboard) destinato ad amministratori di sistema, sviluppatori o power-user che richiedono metriche dettagliate e un'analisi immediata delle prestazioni del sistema.

Il problema che risolve è la necessità di un'interfaccia unificata che consenta l'aggregazione di dati provenienti da fonti hardware diverse e spesso complesse (ad esempio, CPU via WMI, GPU via NVML, sessioni di rete, ecc.). L'applicazione eccelle nel separare la logica pesante di acquisizione dati (realizzata in Rust, garantendo performance e accesso basso livello al sistema operativo) dalla logica di presentazione e reattività (realizzata in React/TypeScript).

Il caso d'uso principale prevede la visualizzazione simultanea di:
1.  **Statistiche di carico attuali:** Carico CPU, utilizzo RAM, temperatura componentistica.
2.  **Dinamiche di sistema:** Grafici temporali (sparklines) di utilizzo e traffico.
3.  **Informazioni statiche/complesse:** Specifiche della scheda madre, dettagli dei processori e delle GPU, cronologia delle sessioni di rete o di lavoro.

L'architettura è intrinsecamente ottimizzata per la latenza minima, utilizzando canali di comunicazione IPC (Inter-Process Communication) efficienti tra il frontend e il backend Rust.

## 🚀 Funzionalità Principali

Le funzionalità dedotte dall'analisi del codice coprono l'intero spettro del monitoraggio hardware e software:

*   **Monitoraggio Dettagliato delle Risorse (Component Cards):**
    *   **CPU Monitoring:** Visualizzazione del carico in tempo reale, conteggio dei core e delle frequenze operanti.
    *   **RAM Monitoring:** Tracciamento dell'utilizzo totale e disponibile della memoria fisica, inclusa la gestione degli stati di default.
    *   **GPU Monitoring:** Acquisizione di metriche specifiche della GPU, inclusi parametri di temperatura, utilizzo della memoria VRAM e performance specifiche per architettura (Nvidia, AMD, Intel).
    *   **Disco I/O Monitoring:** Monitoraggio dell'utilizzo dello spazio su disco e, crucialmente, del tasso di lettura/scrittura (I/O rates) in tempo reale.
    *   **Rete (Network) Monitoring:** Tracciamento del traffico dati in entrata e in uscita, identificazione dell'interfaccia primaria e calcolo delle statistiche di connettività.
*   **Dashboard Compositiva e Grafica:**
    *   **Summary Pills:** Fornisce un riepilogo immediato e ad alta visibilità delle metriche critiche (CPU, RAM, GPU, Network) sotto forma di badge sintetici.
    *   **Component Cards Interattive:** Presentazione tabellare e grafica delle statistiche, ognuna con la propria logica di visualizzazione (es. `ProcessCard` per elenco dettagliato dei processi).
*   **Analisi dei Processi e Sessioni:**
    *   **Process Monitoring:** Elenco dinamico dei processi attivi, con metrica di utilizzo di CPU e memoria, permettendo potenzialmente la gestione (terminazione o sorting).
    *   **Session Tracking:** Gestione e visualizzazione della cronologia delle sessioni di lavoro (es. connessioni utente, sessioni di servizio), con possibilità di avvio, interruzione o cancellazione delle stesse.
*   **Acquisizione e Visualizzazione delle Specifiche di Sistema (System Specs):**
    *   **Hardware Identity:** Raccolta di informazioni statiche e approfondite che includono dettagli sulla scheda madre (MotherboardInfo), i componenti RAM (RamSpecs), la CPU e i dispositivi di visualizzazione (DisplaySpecs). Questo modulo utilizza meccanismi di interrogazione complessi come WMI (Windows Management Instrumentation) per estrarre dati OS-dipendenti.
    *   **Uptime e Tempo:** Calcolo dell'uptime sistemico e formattazione professionale dei tempi di attività.
*   **Gestione e Utility:**
    *   **State Management:** Utilizzo di hook specifici (`useStaticCpuInfo`, `useSystemMonitor`, ecc.) per isolare e gestire lo stato di monitoraggio di ciascun componente, garantendo reattività e memoizzazione dei dati.
    *   **Utility di Rendering:** Funzioni grafiche riutilizzabili (`Sparkline`, `Donut`, `HBar`) che facilitano la visualizzazione di dati temporali e percentuali complesse.

## 🛠️ Architettura e Tecnologie

L'architettura adottata è un **Frontend-Backend Decoupled Monorepo**, implementato con l'approccio del **Hybrid Application** tramite Tauri. Questo schema è fondamentale per garantire che la gestione dei dati critici e l'accesso al sistema operativo siano gestiti in un ambiente a basso livello e performante (Rust), mentre l'interfaccia utente rimanga moderna, reattiva e maneggevole (React/TypeScript).

### Livelli Architetturali:

1.  **Presentation Layer (Frontend):**
    *   **Tecnologie:** React, TypeScript, Vite.
    *   **Funzione:** Interroga lo stato di monitoraggio (`useSystemMonitor`, `useProcessMonitor`), riceve i dati serializzati JSON dal backend tramite IPC, e renderizza i componenti grafici (Card, Charts).
2.  **Logic/State Management Layer (Frontend Hooks):**
    *   **Tecnologie:** Custom Hooks (TypeScript).
    *   **Funzione:** Abstraction layer che gestisce i cicli di vita dei dati, gli aggiornamenti periodici (polling logic) e la conversione dei payload grezzi in stati UI utilizzabili (es. `updateCpuData`).
3.  **System Core Layer (Backend):**
    *   **Tecnologie:** Rust.
    *   **Funzione:** Il cuore del sistema. Contiene i sensori, gli orquestratori e i comandi che interrogano direttamente il SO.
4.  **System Interface Layer (Tauri Commands):**
    *   **Tecnologie:** Rust, `tauri::command`.
    *   **Funzione:** Espone le funzionalità del sistema core (i `Sensors`) al frontend, agendo come un ponte sicuro e sincrono/asincrono per i dati.

### Dipendenze e Dipendenze Cross-Cutting:

*   **A livello di sistema operativo (Rust):** Vengono utilizzate librerie specifiche per interfacciarsi con OS diversi: `sysinfo` per dati generici, `wmi` e `winreg` per dettagli specifici di Windows, e `nvml-wrapper` per l'accesso alle API di monitoraggio Nvidia, dimostrando una sofisticata gestione della portabilità layer-by-layer.
*   **Gestione dei Dati:** I tipi dati (structs e interfaces) sono rigorosamente definiti e mantenuti coerenti tra Rust (`src-tauri/src/types`) e TypeScript (`src/types`), utilizzando i meccanismi di serializzazione `serde` (Rust) per garantire l'integrità del payload JSON scambiato attraverso il canale IPC.

## 🧩 Moduli e Componenti Core

### 🟢 Backend (Rust - `src-tauri/`)

Il backend è organizzato in moduli che separano le responsabilità:

*   **System Interaction:** Contiene le interfacce specifiche per interagire con il sistema operativo (e.g., `src/lib.rs`, `src/command.rs`).
*   **Sensors/Data Acquisition:**
    *   `src/sensor/cpu.rs`, `src/sensor/memory.rs`: Moduli che estraggono metriche di base (CPU, RAM).
    *   `src/sensor/network.rs`: Moduli per lo stato di rete.
*   **Specialized Components (Moduli di Lavoro):**
    *   `src/sensor/process_manager.rs`: Gestisce l'elenco dei processi e il loro consumo (Process/Resource Management).
    *   `src/sensor/gpu.rs`: Interfacce specifiche per le schede grafiche.
*   **Feature Modules:**
    *   `src/command/command.rs`: Il punto di ingresso che orchestra le chiamate ai sensori.

**Componenti di Lavoro (State Management):**

*   **`src/process_manager.rs`**: Strutture dati e logiche per tracciare lo stato dei processi in esecuzione, inclusi i loop di polling.

### Frontend Components (React/TypeScript)

*   **Context/Hooks:**
    *   `useSystemStats`: Un hook che gestisce il polling asincrono delle metriche del sistema (CPU, RAM, etc.) e fornisce i dati aggiornati allo stato globalmente.
*   **Components:**
    *   `SystemDashboard`: Componente principale che aggrega tutti i sottocomponenti.
    *   `ChartComponent`: Utilizza librerie di grafici per visualizzare l'evoluzione temporale delle metriche.
    *   `ResourceGauge`: Componenti UI specifici per mostrare lo stato attuale (e.g., un gauge per l'utilizzo della CPU).

---

### Esempi Di schermate
<p align="center">
   <img width="800" height="600" alt="Screenshot 2026-04-14 231023" src="https://github.com/user-attachments/assets/5a248878-803a-406e-8cf1-0597a90ecec5" />
   <img width="800" height="600" alt="Screenshot 2026-04-14 231211" src="https://github.com/user-attachments/assets/b9e5684d-1dd0-4e97-a23b-27c681654194" />
   <img width="800" height="600" alt="Screenshot 2026-04-14 231159" src="https://github.com/user-attachments/assets/40bf9021-4065-4b93-89b8-06205f7d9503" />
</p>



### Riepilogo della Struttura Dati (Il Ciclo di Vita dei Dati)

1.  **Polling (Rust):** Un thread periodico chiama i sensori (CPU, RAM, GPU) per ottenere un payload JSON aggiornato.
2.  **Comunicazione (Rust <-> JS):** Il payload viene serializzato e inviato al frontend.
3.  **Stato (React Context):** Il payload aggiorna lo stato globale del componente `SystemDashboard`.
4.  **Rendering (React):** I componenti UI (Gauge, Charts) si sottoscrivono allo stato e si ridisegnano automaticamente con i dati freschi.

## Project Structure:
```text
System-monitor/
├── .vscode
├── public
├── src
│   ├── assets
│   ├── components
│   │   ├── cards
│   │   │   ├── CpuCard.tsx
│   │   │   ├── DiskCard.tsx
│   │   │   ├── GpuCard.tsx
│   │   │   ├── NetCard.tsx
│   │   │   ├── ProcessCard.tsx
│   │   │   ├── RamCard.tsx
│   │   │   ├── SessionCard.tsx
│   │   │   ├── SpecsCard.tsx
│   │   │   ├── SummaryPills.tsx
│   │   │   └── index.tsx
│   │   ├── layout
│   │   │   ├── Header.tsx
│   │   │   └── SysBar.tsx
│   │   ├── tab
│   │   │   ├── session
│   │   │   │   ├── SessionHeader.tsx
│   │   │   │   └── SessionPagination.tsx
│   │   │   ├── OverviewTab.tsx
│   │   │   ├── ProcessTab.tsx
│   │   │   ├── SessionTab.tsx
│   │   │   └── SpecsTab.tsx
│   │   └── ui
│   │       └── SharedUi.tsx
│   ├── constants
│   │   └── defaultStates.ts
│   ├── features
│   ├── hooks
│   │   ├── static
│   │   │   ├── useStaticCpuInfo.ts
│   │   │   ├── useStaticDiskInfo.ts
│   │   │   ├── useStaticGpuInfo.ts
│   │   │   ├── useStaticRamInfo.ts
│   │   │   └── useStaticSystemInfo.ts
│   │   ├── useProcessMonitor.ts
│   │   ├── useSessionMonitor.ts
│   │   ├── useSpecsMonitor.ts
│   │   └── useSystemMonitor.ts
│   ├── types
│   │   ├── cpu.ts
│   │   ├── disk.ts
│   │   ├── gpu.ts
│   │   ├── index.ts
│   │   ├── network.ts
│   │   ├── process.ts
│   │   ├── ram.ts
│   │   ├── session.ts
│   │   ├── specs.ts
│   │   └── uptime.ts
│   ├── utils
│   │   ├── adapter.ts
│   │   └── timeFormatters.ts
│   ├── App.tsx
│   ├── main.tsx
│   └── vite-env.d.ts
├── src-tauri
│   ├── capabilities
│   ├── gen
│   │   └── schemas
│   ├── icons
│   │   ├── icon.icns
│   │   └── icon.ico
│   ├── src
│   │   ├── command
│   │   │   ├── cpu_command.rs
│   │   │   ├── disk_command.rs
│   │   │   ├── gpu_command.rs
│   │   │   ├── ram_command.rs
│   │   │   ├── session_command.rs
│   │   │   ├── specs_command.rs
│   │   │   └── system_command.rs
│   │   ├── sensor
│   │   │   ├── gpu_strategy
│   │   │   │   ├── amd_strategy.rs
│   │   │   │   ├── gpu_chain.rs
│   │   │   │   ├── intel_strategy.rs
│   │   │   │   ├── mod.rs
│   │   │   │   └── nvidia_strategy.rs
│   │   │   ├── process
│   │   │   │   ├── mod.rs
│   │   │   │   ├── process_sensor.rs
│   │   │   │   └── sorter.rs
│   │   │   ├── session
│   │   │   │   ├── mod.rs
│   │   │   │   ├── session_data.rs
│   │   │   │   ├── storage.rs
│   │   │   │   └── tracker.rs
│   │   │   ├── sys_complete_info
│   │   │   │   ├── fetcher
│   │   │   │   │   ├── cpu_fetcher.rs
│   │   │   │   │   ├── disk_fetcher.rs
│   │   │   │   │   ├── display_fetcher.rs
│   │   │   │   │   ├── mobo_fetcher.rs
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── os_fetcher.rs
│   │   │   │   │   └── ram_fetcher.rs
│   │   │   │   ├── complete_info_data.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── wmi_data.rs
│   │   │   │   └── wmi_fetcher.rs
│   │   │   ├── cpu_sensor.rs
│   │   │   ├── cpu_temp_sensor.rs
│   │   │   ├── disk_sensor.rs
│   │   │   ├── gpu_sensor.rs
│   │   │   ├── hardware_orchestrator.rs
│   │   │   ├── mod.rs
│   │   │   ├── net_identity.rs
│   │   │   ├── net_sensor.rs
│   │   │   ├── ping_sensor.rs
│   │   │   ├── ram_sensor.rs
│   │   │   ├── sys_info_sensor.rs
│   │   │   └── uptime_sensor.rs
│   │   ├── app_mode.rs
│   │   ├── background_worker.rs
│   │   ├── command.rs
│   │   ├── lib.rs
│   │   └── main.rs
│   ├── Cargo.toml
│   └── build.rs
└── vite.config.ts
```
