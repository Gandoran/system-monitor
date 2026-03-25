export interface SessionResults {
  processName: string | null;
  durationSeconds: number;
  cpuMaxTemp: number;
  cpuAvgTemp: number;
  cpuAvgLoad: number;
  gpuMaxTemp: number;
  gpuAvgTemp: number;
  gpuAvgLoad: number;
  ramAvgLoad: number;
}