import { C, SysmonLogo, TabButton } from "../ui/SharedUi";

interface HeaderProps {
    activeTab: string;
    onTabChange: (tab: string) => void;
}

const TABS = ["Overview", "Processes", "Info", "Session"];
export function Header({ activeTab, onTabChange }: HeaderProps) {
    return (
        <div style={{ display: "flex", alignItems: "center", gap: 11, marginBottom: 13 }}>
            <SysmonLogo />
            <div>
                <div style={{ fontSize: 15, fontWeight: 800, letterSpacing: 2.5, color: C.text }}>SYSMON</div>
                <div style={{ fontSize: 9, color: C.muted, letterSpacing: 1.5 }}>SYSTEM MONITOR</div>
            </div>
            <div style={{ marginLeft: "auto", display: "flex", gap: 4 }}>
                {TABS.map((tabName) => (
                    <TabButton 
                        key={tabName}
                        label={tabName}
                        isActive={activeTab === tabName}
                        onClick={() => onTabChange(tabName)}
                    />
                ))}
            </div>
            
        </div>
    );
}