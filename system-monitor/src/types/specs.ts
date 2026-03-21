export interface OsInfo {
    name: string;
    kernelVersion: string;
}

export interface CpuSpecs {
    architecture: string;
    vendor: string;
    maxClockMhz: number;
}

export interface MoboInfo {
    vendor: string;
    model: string;
    biosVersion: string;
}

export interface RamSpecs {
    speedMts: number;
    formFactor: string;
    manufacturer: string;
}

export interface SystemSpecsPayload {
    os: OsInfo;
    cpu: CpuSpecs;
    mobo: MoboInfo;
    ram: RamSpecs;
}