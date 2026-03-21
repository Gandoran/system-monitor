import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { SystemSpecsPayload } from "../types/specs";

export function useSpecsMonitor() {
    const [specs, setSpecs] = useState<SystemSpecsPayload>();
    useEffect(() => {
        console.log("🔍 1. Hook montato: sto per chiamare Rust...");
        invoke<SystemSpecsPayload>("get_static_specs_info")
            .then((data) => {
                console.log("✅ 2. Dati arrivati da Rust:", data);
                setSpecs(data);
            })
            .catch((error) => {
                console.error("❌ 3. Errore di comunicazione con Rust:", error);
            });
    }, []);
    return specs;
}