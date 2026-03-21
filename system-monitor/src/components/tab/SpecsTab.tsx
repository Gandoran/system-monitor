import { useSpecsMonitor } from "../../hooks/useSpecsMonitor";
import { SpecsCard } from "../cards/SpecsCard"

export function SpecsTab() {
  const specs  = useSpecsMonitor();
  if (!specs) {
    return <div style={{ color: "gray", fontSize: 12 }}>Caricamento specifiche hardware...</div>;
  }
  return (
    <div>
      <SpecsCard specs={specs} />
    </div>
  );
}