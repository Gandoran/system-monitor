// FILE: Header.tsx
import { C } from "../ui/SharedUi";

interface HeaderProps {
    activeTab: string;
    onTabChange: (tab: string) => void;
}

export function Header({ activeTab, onTabChange }: HeaderProps) {
    return (
        <div style={{ display: "flex", alignItems: "center", gap: 12, marginBottom: 18 }}>
            <div style={{ width: 36, height: 36 /* ... */ }}> 📊 </div>
            <div>
                <div style={{ fontSize: 17, fontWeight: 800, color: C.text }}>SYSTEM-MONITOR</div>
            </div>
            <div style={{ marginLeft: "auto", display: "flex", gap: 6 }}>
                {["Overview", "Processes", "History"].map((tabName) => {
                    const isActive = tabName === activeTab;
                    return (
                        <button key={tabName} onClick={() => onTabChange(tabName)} style={{ /* ... */ }}>
                            {tabName}
                        </button>
                    );
                })}
            </div>
        </div>
    );
}