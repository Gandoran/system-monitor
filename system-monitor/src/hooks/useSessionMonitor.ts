import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { SessionResults } from "../types/session";

export function useSessionMonitor() {
    const [isRunning, setIsRunning] = useState(false);
    const [history, setHistory] = useState<SessionResults[]>([]);
    useEffect(() => {
        invoke<SessionResults[]>("get_session_history")
        .then(setHistory)
        .catch((e) => console.error("Errore caricamento storico iniziale:", e));
    }, []);

    const startSession = async () => {
        try {
            await invoke("start_session");
            setIsRunning(true);
        } catch (e) {
            console.error("Errore avvio sessione:", e);
        }
    };

    const stopSession = async () => {
        try {
            await invoke<SessionResults>("stop_session");
            setIsRunning(false);
            const updatedHistory = await invoke<SessionResults[]>("get_session_history");
            setHistory(updatedHistory);
        } catch (e) {
            console.error("Errore stop sessione:", e);
        }
    };
    return {
        isRunning,
        history,
        startSession,
        stopSession
    };
}