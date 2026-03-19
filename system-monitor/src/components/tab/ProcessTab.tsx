import { useProcessMonitor } from "../../hooks/useProcessMonitor";
import { ProcessesCard } from "../cards/ProcessCard"

export function ProcessesTab() {
  const processes = useProcessMonitor();
  return (
    <div>
      <ProcessesCard processes={processes} />
    </div>
  );
}