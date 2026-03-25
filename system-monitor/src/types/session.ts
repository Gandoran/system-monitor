export interface SessionResults {
  processName: string | null;
  durationSeconds: number;
  cpuMaxTemp: number;
  cpuAvgTemp: number;
  cpuMaxLoad: number;
  cpuAvgLoad: number;
  gpuMaxTemp: number;
  gpuAvgTemp: number;
  gpuMaxLoad: number;
  gpuAvgLoad: number;
  ramMaxLoad: number;
  ramAvgLoad: number;
}